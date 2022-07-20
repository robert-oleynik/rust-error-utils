// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use quote::__private::TokenStream;
use syn::DeriveInput;

/// Derive macro implementation to combine multiple errors into one object.
pub struct Collection {
	raw: DeriveInput,
}

impl Collection {
	/// Parse [`DeriveInput`].
	pub fn parse(self) -> Self {
		todo!()
	}

	/// Generate [`TokenStream`] from parse input.
	pub fn generate(self) -> TokenStream {
		todo!()
	}
}

impl From<DeriveInput> for Collection {
	fn from(raw: DeriveInput) -> Self {
		Self {
			raw,
		}
	}
}
