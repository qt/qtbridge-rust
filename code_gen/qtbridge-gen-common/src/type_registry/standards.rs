// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::sync::LazyLock;

use super::cell::CellType;
use super::containers::StandardContainer;
use super::holders::ValueHolder;
use super::pointers::PointerType;
use super::primitives::PrimitiveType;
use super::strings::StringType;
use super::type_traits::{StaticTypeGroup, TypesEnum, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum StandardType {
    Primitive(PrimitiveType),
    String(StringType),
    Container(StandardContainer),
    Holder(ValueHolder),
    Pointer(PointerType),
    Cell(CellType)
}


impl TypesEnum for StandardType {
    fn dyn_type_info(&self) -> &dyn TypeInfo {
        match self {
            Self::Primitive(primitive) => primitive.dyn_type_info(),
            Self::String(string) => string.dyn_type_info(),
            Self::Container(container) => container.dyn_type_info(),
            Self::Holder(holder) => holder.dyn_type_info(),
            Self::Pointer(pointer) => pointer.dyn_type_info(),
            Self::Cell(cell) => cell.dyn_type_info(),
        }
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        match self {
            Self::Primitive(primitive) => primitive.mut_dyn_type_info(),
            Self::String(string) => string.mut_dyn_type_info(),
            Self::Container(container) => container.mut_dyn_type_info(),
            Self::Holder(holder) => holder.mut_dyn_type_info(),
            Self::Pointer(pointer) => pointer.mut_dyn_type_info(),
            Self::Cell(cell) => cell.mut_dyn_type_info(),
        }
    }
}

impl StaticTypeGroup for StandardType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: LazyLock<Vec<StandardType>> = LazyLock::new(|| {
            let mut result = Vec::<StandardType>::new();
            result.extend(PrimitiveType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.extend(StringType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.extend(StandardContainer::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.extend(ValueHolder::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.extend(PointerType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.extend(CellType::get_static_sorted_list().iter()
                .cloned()
                .map(From::from));
            result.sort_unstable_by(|l, r| l.name().cmp(r.name()));
            result
        });

        LIST.as_slice()
    }
}

impl From<PrimitiveType> for StandardType {
    fn from(value: PrimitiveType) -> Self {
        Self::Primitive(value)
    }
}

impl From<StringType> for StandardType {
    fn from(value: StringType) -> Self {
        Self::String(value)
    }
}

impl From<StandardContainer> for StandardType {
    fn from(value: StandardContainer) -> Self {
        Self::Container(value)
    }
}

impl From<ValueHolder> for StandardType {
    fn from(value: ValueHolder) -> Self {
        Self::Holder(value)
    }
}

impl From<PointerType> for StandardType {
    fn from(value: PointerType) -> Self {
        Self::Pointer(value)
    }
}

impl From<CellType> for StandardType {
    fn from(value:CellType) -> Self {
        Self::Cell(value)
    }
}
