// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use quote::__private::TokenStream;
use syn::{Data, DeriveInput};

/// Derive macro implementation to combine multiple errors into one object.
pub struct Collection {
	raw: Option<DeriveInput>,
	variants: Vec<super::Variant>,
}

impl Collection {
	/// Parse [`DeriveInput`].
	pub fn parse(self) -> Self {
		let en = match self.raw.unwrap().data {
			Data::Enum(en) => en,
			_ => panic!("Expected enum type"),
		};

		let variants: Vec<_> = en
			.variants
			.into_iter()
			.map(|variant| super::Variant::from(variant))
			.collect();

		todo!();
	}

	/// Generate [`TokenStream`] from parse input.
	pub fn generate(self) -> TokenStream {
		todo!()
	}
}

impl From<DeriveInput> for Collection {
	fn from(raw: DeriveInput) -> Self {
		Self {
			raw: Some(raw),
			variants: Vec::new(),
		}
	}
}
