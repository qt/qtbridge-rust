// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use insta::assert_snapshot;
use crate::qt_gen_impl::qobject_module_builder;
use qobject_module_builder::{LinkmeSupport, QObjectModuleBuilder};
use quote::quote;
use qtbridge_gen_common::format_code::{format_rust_code, strip_docs};

#[test]
pub fn test() {
    let input = quote! {
        mod some_module {
            #[derive(Default)]
            struct SomeStruct {
            }

            impl SomeStruct {
                fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32 ) -> bool {
                    false
                }

                fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
                    //QVariant::default()
                }

                fn row_count(&self, parent: &QModelIndex) -> i32 {
                    1
                }
            }
        }
    };

    let input_params = quote!{
        Base = QAbstractListModel
    };

    let mut builder = QObjectModuleBuilder::new(LinkmeSupport::Disabled);
    let output = builder.build_token_stream(input, input_params).unwrap();
    let formatted = format_rust_code(&strip_docs(output)).unwrap();
    assert_snapshot!(formatted);
}


#[test]
pub fn test_no_drop() {
    let input = quote! {
        mod some_module {
            #[derive(Default)]
            struct SomeStruct {
            }

            impl SomeStruct {

            }
        }
    };

    let input_params = quote!{
        Base = QAbstractListModel, NoDrop
    };

    let mut builder = QObjectModuleBuilder::new(LinkmeSupport::Disabled);
    let output = builder.build_token_stream(input, input_params).unwrap();
    let formatted = format_rust_code(&strip_docs(output)).unwrap();
    assert_snapshot!(formatted);
}
