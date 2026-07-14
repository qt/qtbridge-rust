// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[doc(hidden)]
pub trait QListImpl<T> {
    fn append(&mut self, value: T);
    fn clear(&mut self);
    fn contains(&self, value: &T) -> bool;
    fn push_back(&mut self, value: T);
    fn remove(&mut self, i: isize);
    fn reserve(&mut self, size: usize);
    fn size(&self) -> isize;
    fn first(&self) -> &T;
    fn last(&self) -> &T;
    fn do_drop(&mut self);
}
