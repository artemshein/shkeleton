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
//! * chrono - dealing with time and date
//!
//! ## Features
//! Shkeleton also defines a few features which extend the dependencies list and APIs.
//!
//! ### CLI feature
//! Additional dependencies:
//! * clap - define your command line arguments parser
//! * sherr - error handling and logger helpers
//! * glob - dealing with glob patterns
//!
//! ### Concurrency feature
//! Additional dependencies:
//! * scoped-pool - define and use a thread pool
//! * num_cpus - get the number of CPUs and cores available
//! * parking_lot - faster syncronization primitives
//! Concurrency feature also defines a facade for RwLock, which allowes to hide an implementation
//! (std::sync::RwLock or parking_lot::RwLock) behind this facade and switch implementation without
//! need to update sources. It could be valuable because the parking_lot implementation
//! lacks "lock poisoning" and may be harder to debug deadlocks.
//!
//! ### Failure feature
//! Enables `fail` feature of the `sherr` dependency and reexports `sherr::failure`.

mod shkeleton;

#[doc(hidden)]
pub extern crate array_tool;
#[doc(hidden)]
pub extern crate byteorder;
#[doc(hidden)]
pub extern crate chrono;
pub extern crate derive_deref;
pub extern crate derive_more;
#[doc(hidden)]
pub extern crate itertools;
pub extern crate lazy_static;
#[doc(hidden)]
pub extern crate regex;
#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[allow(unused_imports)]
pub extern crate sherr;
#[doc(hidden)]
pub extern crate url;

#[cfg(feature = "cli")]
#[doc(hidden)]
pub extern crate clap;
#[cfg(feature = "cli")]
#[doc(hidden)]
pub extern crate glob;

#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub extern crate num_cpus;
#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub extern crate parking_lot;
#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub extern crate scoped_pool;

#[cfg(feature = "failure")]
pub use sherr::failure_derive::*;
pub use sherr::*;
pub use shkeleton::sync;
