// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use qtbridge_build_utils::file_system_utils::{absolute_path, create_dirs, find_all_files, find_files, get_path_from, get_relative_path, normalize_dir_separators, parent_dir, read_file_content, remove_dir_all, remove_file, write_to_file, write_to_file_if_changed};

pub struct RustFileInfo {
    // True if file contains #[cxx_bridge] annotated module.
    pub has_cxx_bridge: bool,

    // Flags saying whether given module is declared as public in local 'mod.rs' file.
    pub is_pub_mod: bool,

    // Vector of reexports to be added to local 'mod.rs'. E.g  "pub mod pub_something;".
    pub local_reexports: Vec<String>,

    // Vector of idents that will be reexported at project level.
    pub global_mod_idents: Vec<String>,
}

pub enum FileType {
    Rust(RustFileInfo),
    Header,
    Cpp,
}

impl FileType {
    pub fn has_cxx_bridge(&self) -> bool {
        match self {
            FileType::Rust(rust_file) => rust_file.has_cxx_bridge,
            _ => false,
        }
    }

    pub fn is_cpp(&self) -> bool {
        matches!(self, FileType::Cpp)
    }
}

pub struct CodeFile {
    code: String,
    input_file: Option<String>,
    file_type: FileType,
}

impl CodeFile {
    pub fn new_rust(code: String, input_file: Option<String>, info: RustFileInfo) -> Self {
        Self {
            code,
            input_file,
            file_type: FileType::Rust(info),
        }
    }

    pub fn new_rust_mod() -> Self {
        Self {
            code: String::new(),
            input_file: None,
            file_type: FileType::Rust(RustFileInfo {
                has_cxx_bridge: false,
                is_pub_mod: false,
                local_reexports: vec![],
                global_mod_idents: vec![]
            })
        }
    }

    pub fn new_cpp(code: String, input_file: Option<String>) -> Self {
        Self {
            code,
            input_file,
            file_type: FileType::Cpp
        }
    }

    pub fn new_header(code: String, input_file: Option<String>) -> Self {
        Self {
            code,
            input_file,
            file_type: FileType::Header,
        }
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn take_code(&mut self) -> String {
        std::mem::take(&mut self.code)
    }
}

pub type FileTree = BTreeMap<PathBuf, CodeFile>;

pub struct GenerateFilesOutput {
    pub files: FileTree,
    pub output_src_dir: PathBuf,
    pub generated_files_bridge_code: String,
    pub generated_files_cpp_code: String
}


// Trait for customization of code generation
pub trait GenerateFiles {
    fn process_file(&mut self, input_path: &Path) -> Result<FileTree, String>;
    fn package_name() -> &'static str;

    fn generate_files(&mut self, input_root: &Path, output_root: &Path, generate_lists_for_build_rs: bool) -> Result<GenerateFilesOutput, String>
        where Self: Sized
    {
        generate_files(input_root, output_root, self, generate_lists_for_build_rs)
    }

    fn place_files(&self, dest_crate_root: &Path, output: &GenerateFilesOutput) -> Result<(), String> {
        place_files(dest_crate_root, output, Self::package_name())
    }
}

fn generate_files<Generator: GenerateFiles>(input_root: &Path, out_root: &Path, generator: &mut Generator, generate_lists_for_build_rs: bool) -> Result<GenerateFilesOutput, String>
{
    let out_src_dir = out_root.join("src");
    let generated_root = out_src_dir.join("generated");

    // Remove folder with all the files if it already exists
    if out_root.is_dir() {
        remove_dir_all(out_root)?;
    }

    // Create directory for files with code
    create_dirs(&generated_root)?;

    // Get the list of input files in directory
    let input_files = find_files(input_root, true,
        |path| path.file_name().is_some_and(|file_name| file_name != "mod.rs"))?;

    // Process input files one by one
    let mut generated_files = FileTree::new();

    for input_path in input_files {

        // Pass the input file to the generator. Produce files output
        let file_tree = generator.process_file(&input_path)?;
        if file_tree.is_empty() {
            continue;
        }

        // Write the output obtained from the generator
        for (mut file_path, mut code_file) in file_tree {
            if file_path.is_absolute() {
                return Err(format!("Input path must be relative to crate root. Got instead: '{}'", file_path.display()));
            }

            file_path = generated_root.join(file_path);
            let parent_dir = parent_dir(&file_path)?;
            if !parent_dir.is_dir() {
                create_dirs(&parent_dir)?;
            }

            write_to_file(&file_path, &code_file.take_code())?;

            generated_files.insert(file_path, code_file);
        }
    }

    // Create 'mod.rs' files to make generated content part of the crate
    let mod_files = create_mod_files(&generated_root, &generated_files)?;
    for mod_path in mod_files {
        generated_files.insert(mod_path, CodeFile::new_rust_mod());
    }

    // // Create build.rs
    let mut files_bridge = String::new();
    let mut files_cpp = String::new();
    if generate_lists_for_build_rs {
        (files_bridge, files_cpp) = generate_generated_files_lists(out_root, &generated_files)?;
    }

    Ok(GenerateFilesOutput {
        files: generated_files,
        output_src_dir: out_src_dir,
        generated_files_bridge_code: files_bridge,
        generated_files_cpp_code: files_cpp

    })
}

fn create_mod_files(generated_root: &Path, generated_files: &FileTree) -> Result<Vec<PathBuf>, String> {

    // Identify directories containing Rust files
    let mut dirs_files = BTreeMap::<PathBuf, Vec<(&PathBuf, &CodeFile)>>::new();
    for (path, code_file) in generated_files {
        let FileType::Rust(_) = code_file.file_type else {
            continue;
        };

        let dir = parent_dir(path)?;
        dirs_files.entry(dir)
            .or_default()
            .push((path, code_file));
    }

    let mut mod_files = Vec::new();
    let mut global_reexports = Vec::new();

    // Create local 'mod.rs' files in every directory containing Rust files
    // Collect global reexport
    for (dir, files) in &dirs_files {
        let mut mod_items = Vec::new();
        let mut local_reexports = Vec::new();
        for (path, code_file) in files {
            let FileType::Rust(rust_file) = &code_file.file_type else {
                unreachable!()
            };

            let filestem = path.file_stem()
                .ok_or_else(|| format!("No filestem in path '{}'", path.display()))?;
            mod_items.push(format!("pub mod {};", filestem.display()));
            local_reexports.extend(rust_file.local_reexports.clone());

            let path_from_gen_root = get_relative_path(dir, generated_root)?
                .components()
                .map(|comp| comp.as_os_str().to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("::");

            for ident in &rust_file.global_mod_idents {
                let mut path = path_from_gen_root.clone();
                if !ident.is_empty() {
                    path.push_str(&format!("::{ident}"));
                }

                global_reexports.push(format!("pub use {path};"));
            }
        }
        mod_items.sort_unstable();
        local_reexports.sort_unstable();

        let mod_path = dir.join("mod.rs");
        let mut mod_content = mod_items.join("\n");

        if !local_reexports.is_empty() {
            mod_content.push_str(&format!("\n{}\n", local_reexports.join("\n")));
        }
        write_to_file(&mod_path, &mod_content)?;
        mod_files.push(mod_path);
    }

    // Sort directories with Rust files from the most deep to the least deep ones
    let zero_depth_path = absolute_path(generated_root)?;
    let get_path_depth = |path: &Path| -> Result<usize, String> {
        Ok(get_relative_path(path, &zero_depth_path)?
            .components()
            .count())
    };

    // Create map where:
    // * key is depth of folder with 'mod.tmp' file in a directory hierarchy relatively to the generated root.
    // * value is vector of paths to 'mod.rs' files at this level.
    // Needed to iterate from most deep leaves to the root of folder tree.

    let mut depth_dirs = BTreeMap::<i32, Vec<PathBuf>>::new();
    for dir in dirs_files.into_keys() {
        let depth = get_path_depth(&dir)?;
        depth_dirs.entry(depth as i32)
            .or_default()
            .push(dir.clone());
    }

    // Iterate folders in descending depth order
    let max_depth = depth_dirs.last_entry()
        .map_or(0, |entry| *entry.key());
    for depth in (2..=max_depth).rev() {
        let Some(dirs) = depth_dirs.get(&depth) else {
            unreachable!();
        };

        // Create Map<ParentFolder, Vec<ChildFolder>>
        let mut parent_children = BTreeMap::<PathBuf, Vec<PathBuf>>::new();
        for dir in dirs {
            let parent_dir = parent_dir(dir)?;
            parent_children.entry(parent_dir)
                .or_default()
                .push(dir.clone());
        }

        // Write intermediate mod.rs files at this depth of file tree
        for (mod_dir, mut mod_subdirs) in parent_children {
            let mod_file_path = if depth > 0 { mod_dir.join("mod.rs") } else { parent_dir(&mod_dir)?.join("generated.rs") };
            mod_subdirs.sort_unstable();

            let mod_content = get_mod_dir_content(&mod_subdirs, &[])?;
            write_to_file(&mod_file_path, &mod_content)?;

            depth_dirs.entry(depth - 1)
                .or_default()
                .push(parent_dir(&mod_file_path)?);
            mod_files.push(mod_file_path);
        }
    }

    // Create src/generated.rs
    {
        let src_dir = parent_dir(generated_root)?;
        let main_modules_dirs = depth_dirs.remove(&1)
            .unwrap_or_default();

        let generated_rs_path = src_dir.join("generated.rs");
        let generated_rs_content = get_mod_dir_content(&main_modules_dirs, &global_reexports)?;

        write_to_file(&generated_rs_path, &generated_rs_content)?;
        mod_files.push(generated_rs_path);
    }


    Ok(mod_files)
}

/// Create content of 'mod.rs' or 'generated.rs' files
/// declaring modules form subfolders in it
fn get_mod_dir_content(dirs: &[PathBuf], reexport: &[String]) -> Result<String, String> {
    let mut decl_lines: Vec<String> = dirs.iter()
        .map(|mod_dir| {
            let mod_name = mod_dir
                .components()
                .next_back()?
                .as_os_str()
                .to_string_lossy();
            Some(format!("pub mod {mod_name};"))
        })
        .collect::<Option<_>>()
        .ok_or_else(|| "Failed to get mod file content".to_string())?;
    decl_lines.sort_unstable();

    let mut content = decl_lines.join("\n");
    if !reexport.is_empty() {
        content.push_str(&format!("\n{}\n",  reexport.join("\n")));
    }
    Ok(content)
}

fn generate_generated_files_lists(crate_dir: &Path, generated_files: &FileTree) -> Result<(String, String), String> {

    let generated_files_bridge = generated_files.iter()
        .filter_map(|(path, file)| if file.file_type.has_cxx_bridge() { Some(path) } else { None })
        .map(|path| Ok(normalize_dir_separators(&get_relative_path(path, crate_dir)?)))
        .collect::<Result<Vec<_>, String>>()?;

    let generated_files_cpp = generated_files.iter()
        .filter_map(|(path, file)| if file.file_type.is_cpp() { Some(path) } else { None })
        .map(|path| Ok(normalize_dir_separators(&get_relative_path(path, crate_dir)?)))
        .collect::<Result<Vec<_>, String>>()?;

    let const_files_bridge = format!("pub const GENERATED_FILES_BRIDGE: [&'static str; {}] = [\n{}];",
        generated_files_bridge.len(),
        generated_files_bridge.iter()
            .fold("".into(), |acc, path| format!("{acc}    \"{}\",\n", path.display())));

    let const_files_cpp = format!("pub const GENERATED_FILES_CPP: [&'static str; {}] = [\n{}];",
        generated_files_cpp.len(),
        generated_files_cpp.iter()
            .fold("".into(), |acc, path| format!("{acc}    \"{}\",\n", path.display())));

    Ok((const_files_bridge, const_files_cpp))
}

fn place_files(dst_crate_root: &Path, output: &GenerateFilesOutput, generator_package: &str) -> Result<(), String> {
    let dst_crate_root = absolute_path(dst_crate_root)?;
    let dst_src_dir = dst_crate_root.join("src");
    let dst_generated_root = dst_src_dir.join("generated");

    // Get the list of files already existing in the destination folder
    let mut existing_files = BTreeSet::new();
    if dst_generated_root.is_dir() {
        existing_files = find_all_files(&dst_generated_root, true)?
            .into_iter()
            .map(|path| get_relative_path(&path, &dst_src_dir))
            .collect::<Result<_, _>>()?;
    }

    // Copy files from OUT_DIR to destination folder if outdated
    for (src_path, code_file) in &output.files {
        let rel_path = get_relative_path(src_path, &output.output_src_dir)?;
        let dst_path = dst_src_dir.join(&rel_path);

        let input_file = match &code_file.input_file {
            Some(input) => {
                let dst_dir = parent_dir(&dst_path)?;
                normalize_dir_separators(&get_path_from(&dst_dir, &PathBuf::from(input))?)
                    .to_string_lossy()
                    .to_string()
            },
            None => String::new(),
        };
        let header = get_header(&input_file, generator_package);
        let content = format!("{header}{}", read_file_content(src_path)?);

        if dst_path.is_file() {
            write_to_file_if_changed(&dst_path, &content)?;
            existing_files.remove(&rel_path);
        }
        else {
            let dst_dir = parent_dir(&dst_path)?;
            if !dst_dir.is_dir() {
                create_dirs(&dst_dir)?;
            }
            write_to_file(&dst_path, &content)?;
        }
    }

    // Remove other files files from src/generated
    existing_files.iter()
        .try_for_each(|rel_path| remove_file(&dst_src_dir.join(rel_path)))?;

    Ok(())
}

pub fn get_header(input_file: &str, generator_package: &str) -> String {
    const COPYRIGHT_HEADER: &str =
r#"// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
"#;
    let mut header = format!("{COPYRIGHT_HEADER}// This file was auto generated");
    if !generator_package.is_empty() {
        header.push_str(&format!(" by {generator_package} package"));
    }
    if !input_file.is_empty() {
        header.push_str(&format!("\n// from the input file://{input_file}"));
    }
    header.push_str("\n\n");
    header
}
