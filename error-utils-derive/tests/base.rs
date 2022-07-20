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
fn impl_from() -> Result<(), ParseError> {
	let i: i32 = "0".parse()?;
	assert_eq!(i, 0);
	let f: f32 = "0.0".parse()?;
	assert_eq!(f, 0.0);
	Ok(())
}

#[test]
fn impl_display() {
	let fail = || -> Result<i32, ParseError> { Ok("".parse()?) };
	if let Err(err) = fail() {
		assert_eq!(
			format!("{}", err),
			"Failed to parse int (Reason: cannot parse integer from empty string)"
		)
	} else {
		unimplemented!()
	}
}

#[derive(Debug, Errors)]
enum CustomError {
	#[error("Custom error 1")]
	CustomError1,
	#[error("Custom error 2")]
	CustomError2,
}

#[test]
fn custom_error() {
	let e = CustomError::CustomError1;
	assert_eq!(format!("{}", e), "Custom error 1");
	let e = CustomError::CustomError2;
	assert_eq!(format!("{}", e), "Custom error 2");
}

#[derive(Debug, Errors)]
enum MultiError {
	#[error("Error: {} {}")]
	Multi(f32, i32),
}

#[test]
fn multi_error() {
	let e = MultiError::Multi(0.0, 0);
	assert_eq!(format!("{}", e), "Error: 0 0")
}
