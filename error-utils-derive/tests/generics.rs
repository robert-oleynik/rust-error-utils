use error_utils_derive::Errors;
use std::num::ParseIntError;

#[derive(Debug, Errors)]
pub enum Generic<T: std::fmt::Debug + std::fmt::Display> {
	#[error("error: {}", from)]
	T(T),
	#[error("custom error")]
	CustomError,
}

#[test]
fn generic() {
	let e = Generic::T(0);
	assert_eq!(format!("{}", e), "error: 0");
}

#[test]
fn generic_from() {
	let fail = || -> Result<i32, Generic<ParseIntError>> { Ok("".parse()?) };
	if let Err(err) = fail() {
		assert_eq!(format!("{}", err), "error: cannot parse integer from empty string")
	} else {
		unimplemented!()
	}
}

#[test]
fn custom_error() {
	let e: Generic<i32> = Generic::CustomError;
	assert_eq!(format!("{}", e), "custom error");
}

#[derive(Debug, Errors)]
pub enum Error<A, B, C>
where
	A: std::error::Error,
	B: std::error::Error,
	C: std::error::Error,
{
	A(A),
	B(B),
	C(C),
}
