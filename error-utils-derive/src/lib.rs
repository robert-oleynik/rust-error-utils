// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use proc_macro::TokenStream;
use syn::DeriveInput;

mod error;

#[proc_macro_derive(Errors, attributes(error))]
pub fn derive_errors(item: TokenStream) -> TokenStream {
	let item = syn::parse_macro_input!(item as DeriveInput);
	error::Collection::from(item).parse().generate().into()
}
