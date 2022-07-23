// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Robert Oleynik

mod fail;
mod handle;

#[cfg(feature = "derive")]
pub use error_utils_derive::Errors;
