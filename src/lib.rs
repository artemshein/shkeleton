//! Shkeleton is a skeleton Rust project which defines some default dependencies and contains some
//! common API's.
//! The idea behind Shkeleton is that you don't need to update all the dependencies by hand for
//! every your library or binary, you could just update Shkeleton version and get all updates.
//! ## Dependencies
//! * log - logging facade
//! * byteorder - dealing with data reading/writing
//! * lazy_static - macro to define a lazy static constants
//! * array_tool - utilities for dealing with arrays
//! * itertools - utilities for dealing with iterators
//! * regex - regular expressions
//! * url - handling URLs
//! * percent_encoding - URL encoding
//! * derive_more & derive_deref - more derive implementations
//! * chrono - dealing with time and date
//! * fstrings - string interpolation macros
//! * fehler - exceptions like approach to errors
//! * sherr - error signalling and logger helpers
//! * log - logging API (through sherr)
//! * anyhow - flexible error signalling (through sherr)
//! * backtrace - backtrace routines (through sherr)
//!
//! ## Features
//! Shkeleton also defines a few features which extend the dependencies list and APIs.
//!
//! ### cli
//! Additional dependencies:
//! * clap - define your command line arguments parser
//! * glob - dealing with glob patterns
//! * dirs - dealing with system paths
//! * fern - logging implementation (through sherr)
//!
//! ### concurrency
//! Additional dependencies:
//! * crossbeam - multi-threading utils
//! * num_cpus - get the number of CPUs and cores available
//! * parking_lot - faster synchronization primitives
//! Concurrency feature also defines a facade for RwLock, which allows to hide an implementation
//! (std::sync::RwLock or parking_lot::RwLock) behind this facade and switch implementation without
//! need to update sources.
//!
//! ### deadlock_detection
//! Enables parking_lot deadlock_detection feature.

pub mod sync;

#[doc(hidden)]
pub use array_tool;
#[doc(hidden)]
pub use byteorder;
#[doc(hidden)]
pub use chrono;
#[doc(hidden)]
pub use derive_deref;
#[doc(hidden)]
pub use derive_more;
#[doc(hidden)]
pub use fehler;
#[doc(hidden)]
pub use fstrings;
#[doc(hidden)]
pub use itertools;
#[doc(hidden)]
pub use lazy_static;
#[doc(hidden)]
pub use percent_encoding;
#[doc(hidden)]
pub use regex;
#[doc(hidden)]
pub use sherr;
#[doc(hidden)]
pub use url;

#[cfg(feature = "cli")]
#[doc(hidden)]
pub use clap;
#[cfg(feature = "cli")]
#[doc(hidden)]
pub use dirs;
#[cfg(feature = "cli")]
#[doc(hidden)]
pub use glob;

#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub use crossbeam;
#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub use num_cpus;
#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub use parking_lot;
