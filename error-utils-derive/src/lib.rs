// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

use proc_macro::TokenStream;
use syn::DeriveInput;

mod error;

/// Derive macro to simplify error type implementation. Implements [`std::fmt::Display`],
/// [`std::error::Error`] for the derived Type.
///
/// # Creating new Error Type
///
/// **Note:** The created type must be an enum.
///
/// ```rust
/// use error_utils_derive::Errors;
/// // or "use error_utils::Errors" if you use the error_utils crate.
///
/// #[derive(Debug, Errors)]
/// enum Error {
///		#[error("Error 1")]
///		Variant1,
///		#[error("Error: {}")]
///		Variant2(i32),
///		// ...
/// }
///
///	assert_eq!(format!("{}", Error::Variant1), "Error 1");
///	assert_eq!(format!("{}", Error::Variant2(42)), "Error: 42");
/// ```
///
/// This will create an enum with name `Error` which implements the [`std::fmt::Display`] and
/// [`std::error::Error`] trait. The output of `format!` depends on the message specified for the
/// variants (For more details see: [Changing The Error Message](#changing-the-error-message)).
///
/// # Changing the Error Message
///
/// As seen above the error message can be changed per enum variant by using the `error` attribute.
/// The default error message of unnamed enum variants (e.g. `Error::Variant2`) with one field is
/// `"{}"`. For all other variants the message must be specified.
///
/// ## Error Message of Unit Variants
///
/// ```rust
/// use error_utils_derive::Errors;
/// // or "use error_utils::Errors" if you use the error_utils crate.
///
/// #[derive(Debug, Errors)]
/// enum Error {
///		// ...
///		#[error("<message>")]
///		UnitVariant
/// }
/// ```
///
/// The usage of `"{}"` inside of `<message>` has no effect on the result of [`std::fmt::Display`].
///
/// ## Error Message of Unnamed Variants
///
/// ```rust
/// use error_utils_derive::Errors;
/// // or "use error_utils::Errors" if you use the error_utils crate.
///
/// #[derive(Debug, Errors)]
/// enum Error {
///		// ...
///		#[error("<message with {}>")]
///		UnnamedVariant(i32)
/// }
/// ```
///
/// The use of `"{}"` is required by the error message. To display brackets use `{{ }}`. For more
/// semantics see [`std::format_args`].
///
/// If you use more than one field in unnamed variants you have to use multiple `{}` for the
/// fields:
///
/// ```rust
/// use error_utils_derive::Errors;
/// // or "use error_utils::Errors" if you use the error_utils crate.
///
/// #[derive(Debug, Errors)]
/// enum Error {
///		// ...
///		#[error("error message with multiple fields: (field 1: {}, field 2: {})")]
///		UnnamedVariant(i32, f32)
/// }
/// ```
///
/// **Note:** Every field of the variant **must** be used.
///
/// ## Error Message of Named Variants
///
/// Named enum variants are not supported yet. Use unnamed enum variants instead (See [Error
/// Message Of Unnamed Variants](#error-message-of-unnamed-variants)).
///
/// # Using `Errors` with `try!` or `?`
///
/// ```rust
/// use error_utils_derive::Errors;
/// // or "use error_utils::Errors" if you use the error_utils crate.
///
/// #[derive(Debug, Errors)]
/// enum Error {
///		// ...
///		#[error(from)]
///		Io(std::io::Error)
/// }
///
/// fn read_file<P: AsRef<std::path::Path>>(path: P) -> Result<(), Error> {
///		let _content = std::fs::read_to_string(path)?;
///		// ...
///		Ok(())
/// }
/// ```
///
/// Passing `from` to the attribute `error` will result in the implementation of `From` for the
/// field of the variant.
///
/// Some limits if you use `from`:
///
/// - `from` can only be used with 1-field unnamed enum variants.
/// - `from` can not be used on multiple enum variants with same field type. (You have to select
///   one of them)
/// - `from` can not be used with templated variants and other enum variants at the same time. (You
///   have to select one of them)
///
/// # Using `Errors` with Type Parameters/Templates
///
/// ```rust
/// use error_utils_derive::Errors;
/// // or "use error_utils::Errors" if you use the error_utils crate.
///
/// #[derive(Debug, Errors)]
/// enum Error<T: std::fmt::Debug + std::fmt::Display> {
///		#[error("{}", from)]
///		E(T),
///		#[error("Some other error")]
///		Other
/// }
/// ```
#[proc_macro_derive(Errors, attributes(error))]
pub fn derive_errors(item: TokenStream) -> TokenStream {
	let item = syn::parse_macro_input!(item as DeriveInput);
	error::Collection::from(item).generate().into()
}
