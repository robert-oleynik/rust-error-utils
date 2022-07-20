// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use syn::*;

pub struct Variant {
	impl_from: bool,
	message: Option<LitStr>,
	ident: Ident,
	fields: Fields,
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
