[![pipeline status](https://gitlab.com/robert-oleynik/rust-error-utils/badges/main/pipeline.svg)](https://gitlab.com/robert-oleynik/rust-error-utils/-/commits/main)
[![Latest Release](https://gitlab.com/robert-oleynik/rust-error-utils/-/badges/release.svg)](https://gitlab.com/robert-oleynik/rust-error-utils/-/releases)

# rust-error-utils

A collection of some rust macros to simplify common error handling patterns.

1. [Usage](#usage)
	1. [`handle_err` Macro](#handle_err-macro)
	2. [`fail` Macro](#fail-macro)
	3. [`Errors` Derive Macro](#errors-derive-macro)
2. [License](#license)

## Usage

Add to `Cargo.toml`:
```toml
[dependencies.error-utils]
git = "https://gitlab.com/robert-oleynik/rust-error-utils.git"
```

### `handle_err` Macro

```rust
use error_utils::handle_err;
use std::path::Path;

fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
	let content = handle_err!(std::fs::read_string(path), err => {
		eprintln!("{}", err);
		return Err(err);
	})
	// ...
}
```

See [Documentation](https://docs.rs/error-utils/latest/error_utils/index.html)

### `fail` Macro

Shorthand for `std::process::exit`

```rust
fail!();
// or
fail!(10);
```

See [Documentation](https://docs.rs/error-utils/latest/error_utils/macro.fail.html)

### `Errors` Derive Macro

>
> This Macro requires the `derive` feature. (Enabled per default)
>

**Note:** This example uses the [`toml`](https://crates.io/crates/toml) crate.

```rust
use std::path::Path;

use error_utils::Errors;

#[derive(Debug, Errors)]
enum ConfigError {
	#[error("Failed to read config file (Reason: {})", from)]
	Io(std::io::Error),
	#[error("Failed to parse config file (Reason: {})", from)]
	Toml(toml::de::Error)
}

fn read_config<P: AsRef<Path>>(path: P) -> Result<toml::Value, ConfigError> {
	let content = std::fs::read_to_string(path)?;
	Ok(toml::from_str(&content)?)
}
```

See [Documentation](https://docs.rs/error-utils/latest/error_utils/macro.Errors.html)

## License

This project is license under the `MIT` license. See
[`LICENSE`](https://gitlab.com/robert-oleynik/rust-error-utils/-/blob/main/LICENSE)
