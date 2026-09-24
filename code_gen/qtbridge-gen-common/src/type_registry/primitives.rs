// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::sync::LazyLock;

use crate::type_registry::type_traits::{GenericArgs, MetaTypeId, StaticTypeGroup, TypesEnum, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrimitiveType {
    Arithmetic(ArithmeticType),
    NonArithmetic(NonArithmeticType),
}

impl TypesEnum for PrimitiveType {
    fn dyn_type_info(&self) -> &dyn TypeInfo {
        match self {
            Self::Arithmetic(arithmetic) => arithmetic.dyn_type_info(),
            Self::NonArithmetic(non_arithmetic) => non_arithmetic.dyn_type_info(),
        }
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        match self {
            Self::Arithmetic(arithmetic) => arithmetic.mut_dyn_type_info(),
            Self::NonArithmetic(non_arithmetic) => non_arithmetic.mut_dyn_type_info(),
        }
    }
}

impl StaticTypeGroup for PrimitiveType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: LazyLock<Vec<PrimitiveType>> = LazyLock::new(|| {
            let mut result = Vec::<PrimitiveType>::new();
            result.extend(ArithmeticType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.extend(NonArithmeticType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.sort_unstable_by(|l, r| l.name().cmp(r.name()));
            result
        });

        LIST.as_slice()
    }
}

impl From<ArithmeticType> for PrimitiveType {
    fn from(value: ArithmeticType) -> Self {
        Self::Arithmetic(value)
    }
}

impl From<NonArithmeticType> for PrimitiveType {
    fn from(value: NonArithmeticType) -> Self {
        Self::NonArithmetic(value)
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ArithmeticType {
    Int(IntType),
    Float(FloatType),
}

impl StaticTypeGroup for ArithmeticType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: LazyLock<Vec<ArithmeticType>> = LazyLock::new(|| {
            let mut result = Vec::<ArithmeticType>::new();
            result.extend(IntType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.extend(FloatType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.sort_unstable_by(|l, r| l.name().cmp(r.name()));
            result
        });

        &LIST
    }
}

impl TypesEnum for ArithmeticType {
    fn dyn_type_info(&self) -> &dyn TypeInfo {
        match self {
            ArithmeticType::Int(int_type) => int_type.dyn_type_info(),
            ArithmeticType::Float(float_type) => float_type.dyn_type_info(),
        }
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        match self {
            ArithmeticType::Int(int_type) => int_type.mut_dyn_type_info(),
            ArithmeticType::Float(float_type) => float_type.mut_dyn_type_info(),
        }
    }
}

impl From<IntType> for ArithmeticType {
    fn from(value: IntType) -> Self {
        Self::Int(value)
    }
}

impl From<FloatType> for ArithmeticType {
    fn from(value: FloatType) -> Self {
        Self::Float(value)
    }
}


#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IntType(ArithmeticTypeData);

impl IntType {
    const fn new(rust_name: &'static str, cpp_name: &'static str, metatype_id: i32) -> Self {
        Self(ArithmeticTypeData::new(rust_name, cpp_name, metatype_id))
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for IntType {
    fn name(&self) -> &str {
        self.0.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl GenericArgs for IntType {}

impl TypeInfo for IntType {
    fn cpp_name(&self) -> Option<&'static str> {
        Some(self.0.cpp_name)
    }

    fn cpp_include(&self) -> Option<String> {
        Some("<cstdint>".into())
    }

    fn metatype_id(&self) -> MetaTypeId {
        self.0.metatype_id.into()
    }
}

impl StaticTypeGroup for IntType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [IntType; 10] = [
            IntType::new("i16",   "int16_t",   33),
            IntType::new("i32",   "int32_t",   2),
            IntType::new("i64",   "int64_t",   4),
            IntType::new("i8",    "int8_t",    40),
            IntType::new("u16",   "uint16_t",  36),
            IntType::new("u32",   "uint32_t",  3),
            IntType::new("u64",   "uint64_t",  5),
            IntType::new("u8",    "uint8_t",   37),

            #[cfg(target_pointer_width = "64")]
            IntType::new("isize", "ptrdiff_t", 4),
            #[cfg(target_pointer_width = "64")]
            IntType::new("usize", "size_t", 5),

            #[cfg(target_pointer_width = "32")]
            IntType::new("isize", "ptrdiff_t", 2),
            #[cfg(target_pointer_width = "32")]
            IntType::new("usize", "size_t", 3),
        ];

        &LIST
    }
}


#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FloatType(ArithmeticTypeData);

impl FloatType {
    const fn new(rust_name: &'static str, cpp_name: &'static str, metatype_id: i32) -> Self{
        Self(ArithmeticTypeData { rust_name, cpp_name, metatype_id })
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for FloatType {
    fn name(&self) -> &str {
        self.0.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl GenericArgs for FloatType {}

impl TypeInfo for FloatType {
    fn cpp_name(&self) -> Option<&'static str> {
        Some(self.0.cpp_name)
    }

    fn metatype_id(&self) -> MetaTypeId {
        self.0.metatype_id.into()
    }
}

impl StaticTypeGroup for FloatType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [FloatType; 2] = [
            FloatType::new("f32", "float", 38),
            FloatType::new("f64", "double", 6),
        ];

        &LIST
    }
}



#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ArithmeticTypeData {
    rust_name: &'static str,
    cpp_name: &'static str,
    metatype_id: i32,
}


impl ArithmeticTypeData {
    const fn new(rust_name: &'static str, cpp_name: &'static str, metatype_id: i32) -> Self {
        Self { rust_name, cpp_name, metatype_id }
    }
}

#[derive(Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonArithmeticType {
    rust_name: &'static str,
    cpp_name: &'static str,
    include: &'static str,
    metatype_id: i32,
}

impl NonArithmeticType {
    const fn new(rust_name: &'static str, cpp_name: &'static str, include: &'static str, metatype_id: i32) -> Self {
        Self { rust_name, cpp_name, include, metatype_id }
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for NonArithmeticType {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl GenericArgs for NonArithmeticType {}

impl TypeInfo for NonArithmeticType {
    fn cpp_name(&self) -> Option<&'static str> {
        Some(self.cpp_name)
    }

    fn cpp_include(&self) -> Option<String> {
        (!self.include.is_empty())
            .then_some(self.include.into())
    }

    fn metatype_id(&self) -> MetaTypeId {
        self.metatype_id.into()
    }
}

impl StaticTypeGroup for NonArithmeticType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [NonArithmeticType; 2] = [
            NonArithmeticType::new("bool", "bool", "", 1),

            // TODO: implement conversion Rust char -> QChar
            NonArithmeticType::new("char", "char_32_t", "<uchar.h>", -1),
        ];
        &LIST
    }
}
