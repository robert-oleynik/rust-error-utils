// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use quote::__private::TokenStream;
use syn::{Data, DeriveInput, Ident};

use super::Generics;

/// Derive macro implementation to combine multiple errors into one object.
pub struct Collection {
	ident: Ident,
	generics: Generics,
	variants: Vec<super::Variant>,
}

impl Collection {
	/// Generate [`TokenStream`] from parse input.
	pub fn generate(self) -> TokenStream {
		let impl_from_variants = self
			.variants
			.iter()
			.filter(|variant| variant.is_impl_from())
			.map(|variant| variant.generate_from(&self.ident, &self.generics));
		let ident = &self.ident;
		let generics_lhs = self.generics.with_bounds();
		let generics_rhs = self.generics.without_bounds();
		let where_clause = self.generics.with_bounds();
		let impl_display_variants = self.variants.iter().map(|v| v.generate_display());
		quote::quote!(
			#( #impl_from_variants )*

			impl #generics_lhs ::std::fmt::Display for #ident #generics_rhs #where_clause {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result<()> {
					match self {
						#( #impl_display_variants ),*
					}
				}
			}

			impl #generics_lhs ::std::error::Error for #ident #generics_rhs #where_clause {}
		)
	}
}

impl From<DeriveInput> for Collection {
	fn from(raw: DeriveInput) -> Self {
		let en = match raw.data {
			Data::Enum(en) => en,
			_ => panic!("Expected enum type"),
		};

		let ident = raw.ident;
		let variants: Vec<_> = en
			.variants
			.into_iter()
			.map(|variant| super::Variant::from(variant))
			.collect();

		Self {
			ident,
			generics: Generics::from(raw.generics),
			variants,
		}
	}
}
