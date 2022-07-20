// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use quote::__private::TokenStream;
use syn::*;

pub struct Variant {
	impl_from: bool,
	message: Option<LitStr>,
	ident: Ident,
	fields: Fields,
}

impl Variant {
	/// Returns `true` if implementation of `From<...>` should be done for this variant.
	pub fn is_impl_from(&self) -> bool {
		self.impl_from
	}

	/// Generate `From` implementation for this variant.
	///
	/// # Parameter
	/// - `ident` Identifier of parent enum.
	pub fn generate_from(&self, ident: &Ident, generics: &super::Generics) -> TokenStream {
		let generics_lhs = generics.with_bounds();
		let generics_rhs = generics.without_bounds();
		let where_clause = generics.where_clause();
		let ty = match &self.fields {
			Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields.unnamed.first().unwrap(),
			Fields::Unnamed(_) => {
				todo!("Only unnamed variants with 1 field support option `from`")
			},
			_ => unimplemented!("Only unnamed variants support option `from`"),
		};
		let va_ident = &self.ident;
		quote::quote!(
			impl #generics_lhs ::std::convert::From < #ty > for #ident #where_clause #generics_rhs {
				fn from(e: #ty) -> Self {
					Self :: #va_ident (e)
				}
			}
		)
	}
}

impl From<syn::Variant> for Variant {
	fn from(variant: syn::Variant) -> Self {
		let mut impl_from = false;
		let mut message = None;
		for option in variant
			.attrs
			.into_iter()
			.map(|attr| attr.parse_meta().unwrap())
			.filter_map(|meta| match meta {
				Meta::List(list) => Some(list),
				_ => None,
			})
			.filter(|meta| meta.path.is_ident("error"))
			.flat_map(|meta| meta.nested.into_iter())
		{
			match option {
				NestedMeta::Lit(Lit::Str(lit)) => {
					if let Some(_) = &message {
						panic!("Message already set for variant {}", variant.ident)
					} else {
						message = Some(lit)
					}
				},
				NestedMeta::Meta(Meta::Path(path)) if path.is_ident("from") => {
					if impl_from {
						panic!("Option `from` already set for variant {}", variant.ident)
					} else {
						impl_from = true
					}
				},
				_ => panic!("Unexpected option {:?} for variant {}", option, variant.ident),
			}
		}
		Self {
			impl_from,
			message,
			ident: variant.ident,
			fields: variant.fields,
		}
	}
}
