////////////////////////////////////////////////////////////////////////////////
// Stall configuration management utility
////////////////////////////////////////////////////////////////////////////////
// Copyright 2020 Skylor R. Schermer
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Error types.
////////////////////////////////////////////////////////////////////////////////

// Exports.
pub use anyhow::Error;
pub use anyhow::Context;

// Standard library imports.
use std::path::Path;


////////////////////////////////////////////////////////////////////////////////
// InvalidFile
////////////////////////////////////////////////////////////////////////////////
/// The specified file was invalid.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct InvalidFile;

impl std::error::Error for InvalidFile {}

impl std::fmt::Display for InvalidFile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
		-> Result<(), std::fmt::Error> 
	{
		write!(f, "Invalid file.")
	}
}



////////////////////////////////////////////////////////////////////////////////
// MissingFile
////////////////////////////////////////////////////////////////////////////////
/// The specified file was missing.
#[derive(Debug, Clone)]
pub struct MissingFile { 
	/// The path of the missing file.
	pub path: Box<Path>,
}

impl std::error::Error for MissingFile {}

impl std::fmt::Display for MissingFile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
		-> Result<(), std::fmt::Error> 
	{
		write!(f, "missing file: {}.", self.path.display())
	}
}
