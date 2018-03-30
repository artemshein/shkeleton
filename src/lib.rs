//! Shkeleton is a skeleton Rust project which defines some default dependencies and contains some
//! common API's.
//! The idea behind a Shkeleton is that you don't need to update all the dependencies by hand for
//! every your library or binary, you could just update a Shkeleton version and get all updates.
//! ## Dependencies
//! * log -- logging facade
//! * byteorder -- dealing with data reading/writing
//! * lazy_static -- macro to define a lazy static constants
//! * array_tool -- utilities for dealing with arrays
//! * itertools -- utilities for dealing with iterators
//! * regex -- regular expressions
//! * url -- handling URLs
//! ## Features
//! Shkeleton also defines a few features which extend the dependencies list and APIs.
//!
//! ### CLI feature
//! Additional dependencies:
//! * clap -- define your command line arguments parser
//!
//! ### Concurrency feature
//! Additional dependencies:
//! * scoped-pool -- define and use a thread pool
//! * num_cpus -- get the number of CPUs and cores available
//! * parking_lot -- faster syncronization primitives
//! Concurrency feature also defines a facade for RwLock, which allowes to hide an implementation
//! (std::sync::RwLock or parking_lot::RwLock) behind this facade and switch implementation without
//! the need to update sources. It could be valuable because the parking_lot implementation
//! lacks "lock poisoning" and maybe harder to debug deadlocks.
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

#[cfg(feature = "cli")]
pub extern crate clap;

#[cfg(feature = "concurrency")]
pub extern crate scoped_pool;
#[cfg(feature = "concurrency")]
pub extern crate num_cpus;
#[cfg(feature = "concurrency")]
pub extern crate parking_lot;

pub use shkeleton::sync;