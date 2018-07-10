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
//! * derive_more & derive_deref - more derive implementations
//!
//! ## Features
//! Shkeleton also defines a few features which extend the dependencies list and APIs.
//!
//! ### CLI feature
//! Additional dependencies:
//! * clap - define your command line arguments parser
//! * fern - complex logger implementation
//! * [deprecated] simple_logger - simple logger implementation
//! * glob - dealing with glob patterns
//!
//! ### Concurrency feature
//! Additional dependencies:
//! * scoped-pool - define and use a thread pool
//! * num_cpus - get the number of CPUs and cores available
//! * parking_lot - faster syncronization primitives
//! Concurrency feature also defines a facade for RwLock, which allowes to hide an implementation
//! (std::sync::RwLock or parking_lot::RwLock) behind this facade and switch implementation without
//! the need to update sources. It could be valuable because the parking_lot implementation
//! lacks "lock poisoning" and maybe harder to debug deadlocks.
//!
//! ### Limitations
//! Due to current Rust macro system limitations in order to use derive macros from the derive_deref
//! or derive_more crates you need to import them manually:
//!
//! ```ignore
//! #[macro_use]
//! extern crate derive_more;
//! #[macro_use]
//! extern crate derive_deref;
//! ```
//!
mod shkeleton;

#[allow(unused_imports)]
#[macro_use]
pub extern crate log;
pub extern crate byteorder;
#[allow(unused_imports)]
#[macro_use]
pub extern crate itertools;
pub extern crate array_tool;
pub extern crate regex;
pub extern crate url;
#[allow(unused_imports)]
#[macro_use]
pub extern crate lazy_static;

#[cfg(feature = "cli")]
pub extern crate clap;
#[cfg(feature = "cli")]
pub extern crate simple_logger;
#[cfg(feature = "cli")]
pub extern crate fern;
#[cfg(feature = "cli")]
pub extern crate glob;

#[cfg(feature = "concurrency")]
pub extern crate scoped_pool;
#[cfg(feature = "concurrency")]
pub extern crate num_cpus;
#[cfg(feature = "concurrency")]
pub extern crate parking_lot;

pub use shkeleton::sync;

pub use lazy_static::*;
pub use log::*;
pub use byteorder::*;
pub use itertools::*;
pub use array_tool::*;
pub use regex::*;
pub use url::*;

#[cfg(feature = "cli")]
pub use clap::*;
#[cfg(feature = "cli")]
pub use simple_logger::*;
#[cfg(feature = "cli")]
pub use fern::*;
#[cfg(feature = "cli")]
pub use glob::*;

#[cfg(feature = "concurrency")]
pub use scoped_pool::*;
#[cfg(feature = "concurrency")]
pub use num_cpus::*;
#[cfg(feature = "concurrency")]
pub use parking_lot::*;