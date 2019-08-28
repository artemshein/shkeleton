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
//! * log - logging API
//! * snafu - handling errors
//!
//! ## Features
//! Shkeleton also defines a few features which extend the dependencies list and APIs.
//!
//! ### CLI feature
//! Additional dependencies:
//! * clap - define your command line arguments parser
//! * glob - dealing with glob patterns
//! * dirs - dealing with system paths
//! * fern - logging implementation
//!
//! ### Concurrency feature
//! Additional dependencies:
//! * scoped-pool - define and use a thread pool
//! * num_cpus - get the number of CPUs and cores available
//! * parking_lot - faster synchronization primitives
//! Concurrency feature also defines a facade for RwLock, which allows to hide an implementation
//! (std::sync::RwLock or parking_lot::RwLock) behind this facade and switch implementation without
//! need to update sources. It could be valuable because the parking_lot implementation
//! lacks "lock poisoning" and may be harder to debug deadlocks.
//!

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
pub use itertools;
#[doc(hidden)]
pub use lazy_static;
#[doc(hidden)]
pub use regex;
#[doc(hidden)]
pub use snafu;
#[doc(hidden)]
pub use url;
#[doc(hidden)]
pub use log;

#[cfg(feature = "cli")]
#[doc(hidden)]
pub use clap;
#[cfg(feature = "cli")]
#[doc(hidden)]
pub use glob;
#[cfg(feature = "cli")]
#[doc(hidden)]
pub use dirs;
#[cfg(feature = "cli")]
#[doc(hidden)]
pub use fern;

#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub use num_cpus;
#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub use parking_lot;
#[cfg(feature = "concurrency")]
#[doc(hidden)]
pub use scoped_pool;
