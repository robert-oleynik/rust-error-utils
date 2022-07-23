// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

/// Shorthand to handle `Option<T>` and `Result<T, E>`
///
/// # Usage
///
/// ## Handling `Option<T>`
///
/// ```rust
/// handle_err!(/* Some expression */, None => /* Some handle */);
/// // Shorthand for
/// let result = match /* Some expression */ {
///		Some(result) => result,
///		None => /* Some handle */,
/// };
/// ```
///
/// ## Handling `Result<T, E>`
///
/// ```rust
/// handle_err!(/* Some expression */, err => /* Some handle */);
/// // Shorthand for
/// let result = match /* Some expression */ {
///		Ok(result) => result,
///		Err(err) => /* Some handle */,
/// };
/// ```
#[macro_export]
macro_rules! handle_err {
	( $e:expr, None => $handle:expr ) => {
		match $e {
			Some(result) => result,
			None => $handle,
		}
	};
	( $e:expr, $err:ident => $handle:expr ) => {
		match $e {
			Ok(result) => result,
			Err($err) => $handle,
		}
	};
}
