// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

/// Shorthand for [`std::process::exit`]
///
/// # Usage
///
/// ```rust
/// fail!();
/// // Shorthand for
/// std::process::exit(1);
/// ```
///
/// or
///
/// ```rust
/// fail!(/* exit code */);
/// // Shorthand for
/// std::process::exit(/* exit code */);
/// ```
#[macro_export]
macro_rules! fail {
	() => {
		::std::process::exit(1)
	};
	( $code:expr ) => {
		::std::process::exit($code)
	};
}
