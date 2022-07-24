// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use quote::__private::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, GenericParam, WhereClause};

/// Abstraction to handle generic arguments.
pub struct Generics {
	params: Punctuated<GenericParam, Comma>,
	where_clause: Option<WhereClause>,
}

impl Generics {
	/// Generate generic parameter with bound. (e.g. `< A: Display, B: Debug >`). Returns `None` if
	/// no generics exists.
	pub fn with_bounds(&self) -> Option<TokenStream> {
		if self.params.is_empty() {
			return None;
		}
		let params = self.params.iter();
		Some(quote::quote!(
			< #( #params ),* >
		))
	}

	/// Generate generic parameter with bound. (e.g. `< A, B >`). Returns `None` if no generics
	/// exists.
	pub fn without_bounds(&self) -> Option<TokenStream> {
		if self.params.is_empty() {
			return None;
		}
		let params = self.params.iter().cloned().map(|param| match param {
			GenericParam::Type(mut ty) => {
				ty.bounds = Punctuated::new();
				GenericParam::Type(ty)
			},
			_ => param,
		});
		Some(quote::quote!(
			< #( #params ),* >
		))
	}

	/// Returns reference to where clause. Returns `None` if no where clause exists.
	pub fn where_clause(&self) -> Option<&WhereClause> {
		self.where_clause.as_ref()
	}
}

impl From<syn::Generics> for Generics {
	fn from(generics: syn::Generics) -> Self {
		Self {
			params: generics.params,
			where_clause: generics.where_clause,
		}
	}
}
