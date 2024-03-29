////////////////////////////////////////////////////////////////////////////////
// Stall configuration management utility
////////////////////////////////////////////////////////////////////////////////
// This code is dual licensed using the MIT or Apache 2 license.
// See license-mit.md and license-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Command options and dispatch.
////////////////////////////////////////////////////////////////////////////////
#![warn(missing_docs)]

// Internal modules.
mod add;
mod collect;
mod distribute;
mod init;
mod remove;
mod rename;
mod status;

// Exports.
pub use add::*;
pub use collect::*;
pub use distribute::*;
pub use init::*;
pub use remove::*;
pub use rename::*;
pub use status::*;


// External library imports.
use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

// Standard library imports.
use std::path::Path;
use std::path::PathBuf;



////////////////////////////////////////////////////////////////////////////////
// CommonOptions
////////////////////////////////////////////////////////////////////////////////
/// Command line options shared between subcommands.
#[derive(Debug, Clone)]
#[derive(Parser)]
#[clap(name = "stall")]
#[allow(clippy::struct_excessive_bools)]
pub struct CommonOptions {
	/// The application configuration file to load.
	#[clap(
		long = "config",
		parse(from_os_str),
		hide(true))]
	pub config: Option<PathBuf>,

	/// The user preferences file to load.
	#[clap(
		long = "prefs",
		parse(from_os_str),
		hide(true))]
	pub prefs: Option<PathBuf>,
	
	/// Shorten filenames by omitting path prefixes.
	#[clap(
		short = 'o',
		long = "short-names")]
	pub short_names: bool,
	
	/// Promote any warnings into errors and abort.
	#[clap(long = "error")]
	pub promote_warnings_to_errors: bool,

	/// When to color output.
	#[clap(
		long = "color",
		default_value = "auto",
		arg_enum)]
	pub color: ColorOption,
	
	/// Provide more detailed messages.
	#[clap(
		short = 'v',
		long = "verbose",
		group = "verbosity")]
	pub verbose: bool,

	/// Silence all non-error program output.
	#[clap(
		short = 'q',
		long = "quiet",
		alias = "silent",
		group = "verbosity")]
	pub quiet: bool,

	/// Print trace messages.
	#[clap(
		long = "ztrace",
		hide(true))]
	pub trace: bool,
}


////////////////////////////////////////////////////////////////////////////////
// CommandOptions
////////////////////////////////////////////////////////////////////////////////
/// Command line subcommand options.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
#[derive(Parser)]
#[clap(name = "stall")]
#[clap(author, version, about, long_about = None)]
pub enum CommandOptions {
	/// Intitialize a stall directory by generating a stall file.
	Init {
		/// Common command options.
		#[clap(flatten)]
		common: CommonOptions,

		/// The stall file to create.
		#[clap(parse(from_os_str))]
		stall: Option<PathBuf>,

		/// Print intended operations instead of running them.
		#[clap(long = "dry-run")]
		dry_run: bool,

		// TODO: Set rename policy
		// TODO: Create prefs file?
	},

	/// Print the status of stalled files.
	Status {
		/// Common command options.
		#[clap(flatten)]
		common: CommonOptions,

		/// The stall file or directory.
		#[clap(
			short = 's',
			long = "stall",
			parse(from_os_str))]
		stall: Option<PathBuf>,

		// TODO: Filter entries?
	},

	// TODO: Add Diff subcommand.

	/// Add files to a stall.
	Add {
		/// Common command options.
		#[clap(flatten)]
		common: CommonOptions,

		/// The stall file or directory.
		#[clap(
			short = 's',
			long = "stall",
			parse(from_os_str))]
		stall: Option<PathBuf>,

		/// The files to add to the stall.
		#[clap(parse(from_os_str))]
		files: Vec<PathBuf>,

		/// Rename the file within the stall. Cannot be used if multiple files
		/// are added.
		#[clap(
			long = "rename",
			parse(from_os_str))]
		rename: Option<PathBuf>,

		/// Add stall files to a subdirectory.
		#[clap(
			long = "into",
			parse(from_os_str))]
		into: Option<PathBuf>,

		/// Immediately collect the added files.
		#[clap(
			short = 'c',
			long = "collect")]
		collect: bool,

		/// Print intended operations instead of running them.
		#[clap(long = "dry-run")]
		dry_run: bool,

		// TODO: Rename multiple files. Needs some kind of 'file iterator 
		// naming schema'
		// TODO: Rename if exists? Needs some kind of 'backup naming schema.'
	},

	/// Remove files from a stall.
	#[clap(name = "rm")]
	Remove {
		/// Common command options.
		#[clap(flatten)]
		common: CommonOptions,

		/// The stall file or directory.
		#[clap(
			short = 's',
			long = "stall",
			parse(from_os_str))]
		stall: Option<PathBuf>,
		

		/// The files to remove from to the stall.
		#[clap(parse(from_os_str))]
		files: Vec<PathBuf>,

		/// Delete the stalled file copy.
		#[clap(
			short = 'd',
			long = "delete")]
		delete: bool,

		/// Select files do delete based on their remote paths instead of their
		/// paths within the stall directory.
		#[clap(long = "remote-naming")]
		remote_naming: bool,

		/// Print intended operations instead of running them.
		#[clap(long = "dry-run")]
		dry_run: bool,

		// TODO: Support glob naming?
	},

	/// Rename a file in a stall. Future collect/distribute actions will use
	/// the new name.
	#[clap(name = "mv")]
	Move {
		/// Common command options.
		#[clap(flatten)]
		common: CommonOptions,

		/// The stall file or directory.
		#[clap(
			short = 's',
			long = "stall",
			parse(from_os_str))]
		stall: Option<PathBuf>,

		/// The current name of the file in the stall.
		#[clap(parse(from_os_str))]
		from: PathBuf,

		/// The new name of the file in the stall.
		#[clap(parse(from_os_str))]
		to: PathBuf,

		/// Move the stalled file copy.
		#[clap(
			short = 'm',
			long = "move")]
		move_file: bool,

		/// Force copy even if files exist.
		#[clap(
			short = 'f',
			long = "force")]
		force: bool,

		/// Print intended operations instead of running them.
		#[clap(long = "dry-run")]
		dry_run: bool,
	},

	/// Copy files into the stall directory from their remote locations.
	Collect {
		/// Common command options.
		#[clap(flatten)]
		common: CommonOptions,

		/// The stall file or directory.
		#[clap(
			short = 's',
			long = "stall",
			parse(from_os_str))]
		stall: Option<PathBuf>,
		

		/// Specific files to collect. Defaults to all files.
		#[clap(parse(from_os_str))]
		files: Vec<PathBuf>,

		/// Force copy even if files are unmodified.
		#[clap(
			short = 'f',
			long = "force")]
		force: bool,

		/// Print intended operations instead of running them.
		#[clap(long = "dry-run")]
		dry_run: bool,
	},

	/// Copi files from the stall directory to their remote locations.
	Distribute {
		/// Common command options.
		#[clap(flatten)]
		common: CommonOptions,

		/// The stall file or directory.
		#[clap(
			short = 's',
			long = "stall",
			parse(from_os_str))]
		stall: Option<PathBuf>,
		

		/// Specific files to distribute. Defaults to all files.
		#[clap(parse(from_os_str))]
		files: Vec<PathBuf>,

		/// Force copy even if files are unmodified.
		#[clap(
			short = 'f',
			long = "force")]
		force: bool,

		/// Print intended operations instead of running them.
		#[clap(long = "dry-run")]
		dry_run: bool,
	},
}

impl CommandOptions {
	/// Returns true if the command is an `Init` variant.
	#[must_use]
	pub fn is_init(&self) -> bool {
		matches!(self, CommandOptions::Init { .. })
	}

	/// Returns the provided stall path, if any.
	#[must_use]
	pub fn stall(&self) -> Option<&Path> {
		use CommandOptions::*;
		match self {
			Init { stall, .. }       |
			Status { stall, .. }     |
			Add { stall, .. }        |
			Remove { stall, .. }     |
			Move { stall, .. }       |
			Collect { stall, .. }    |
			Distribute { stall, .. } => stall.as_deref(),
		}
	}

	/// Returns the `CommonOptions`.
	#[must_use]
	pub fn common(&self) -> &CommonOptions {
		use CommandOptions::*;
		match self {
			Init { common, .. }       |
			Status { common, .. }     |
			Add { common, .. }        |
			Remove { common, .. }     |
			Move { common, .. }       |
			Collect { common, .. }    |
			Distribute { common, .. } => common,
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// ColorOption
////////////////////////////////////////////////////////////////////////////////
/// Options for handling missing files.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[derive(clap::ArgEnum)]
pub enum ColorOption {
	/// Color usage is automatically determined based on environment variables
	/// and TTY usage.
	Auto,
	/// Color output should always be generated.
	Always,
	/// Color output should never be generated.
	Never,
}

impl ColorOption {
	/// Returns true if colored output should be used.
	#[must_use]
	pub fn enabled(&self) -> bool {
		match self {
			Self::Auto => {
				// Defer to `colored` for enviroment vars and TTY detection.
				colored::control::SHOULD_COLORIZE.should_colorize()
			},
			Self::Always => true,
			Self::Never => false,
		}
	}
}

impl std::str::FromStr for ColorOption {
	type Err = ColorOptionParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.eq_ignore_ascii_case("auto") {
			Ok(Self::Auto)
		} else if s.eq_ignore_ascii_case("always") {
			Ok(Self::Always)
		} else if s.eq_ignore_ascii_case("never") {
			Ok(Self::Never)
		} else {
			Err(ColorOptionParseError)
		}
	}
}

/// An error indicating a failure to parse a [`ColorOption`].
///
/// [`ColorOption`]: ColorOption 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorOptionParseError;

impl std::error::Error for ColorOptionParseError {}

impl std::fmt::Display for ColorOptionParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "failure to parse ColorOption")
	}
}
