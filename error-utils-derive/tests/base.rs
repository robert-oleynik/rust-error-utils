use std::num::{ParseFloatError, ParseIntError};

use error_utils_derive::Errors;

#[derive(Debug, Errors)]
enum ParseError {
	#[error("Failed to parse int (Reason: {})", from)]
	ParseInt(ParseIntError),
	#[error("Failed to parse float (Reason: {})", from)]
	ParseFloat(ParseFloatError),
}

#[test]
fn test_impl_from() -> Result<(), ParseError> {
	let i: i32 = "0".parse()?;
	println!("{}", i);
	let f: f32 = "0.0".parse()?;
	println!("{}", f);
	Ok(())
}
