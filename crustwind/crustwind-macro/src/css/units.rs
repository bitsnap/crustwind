/*
 * Copyright (C) 2016-2025 Yuriy Yarosh
 * All rights reserved.
 *
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use proc_macro::TokenStream;

/// Struct to parse the input to the css_unit macro
struct UnitDefinition {
    name: syn::Ident,
    suffix: String,
}

impl syn::parse::Parse for UnitDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let suffix_lit = input.parse::<syn::LitStr>()?;

        Ok(UnitDefinition {
            name,
            suffix: suffix_lit.value(),
        })
    }
}

/// Generates a unit struct for a CSS unit.
pub(crate) fn unit(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let unit_def = syn::parse2::<UnitDefinition>(input).expect("Failed to parse unit definition");

    let unit_name = &unit_def.name;
    let unit_suffix = &unit_def.suffix;
    let display_format = format!("{{_0}}{}", unit_suffix);
    let unit_doc = format!("Represents a CSS `{}` unit.", unit_suffix);

    let output = quote::quote! {
        #[doc = #unit_doc]
        #[derive(derive_more::Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Serialize, Deserialize)]
        #[display(#display_format)]
        pub struct #unit_name(pub i32);
        impl Deref for #unit_name {
            type Target = i32;

            /// Returns a reference to the inner value.
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for #unit_name {
            /// Returns a mutable reference to the inner value.
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<i32> for #unit_name {
            /// Creates a new value from an integer.
            fn from(value: i32) -> Self {
                #unit_name(value)
            }
        }

        impl FromStr for #unit_name {
            type Err = anyhow::Error;

            /// Parses a string into a value, removing the unit suffix if present.
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(#unit_name(s.replace(#unit_suffix, "").trim().parse().unwrap()))
            }
        }
    };

    output.into()
}
