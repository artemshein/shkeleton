extern crate shkeleton;

use shkeleton::{
    byteorder::ReadBytesExt,
    derive_deref::Deref,
    derive_more::From,
    lazy_static::lazy_static,
    sherr::{diag, diag_backtrace, diag_position, diag_unreachable, error, log::info},
    sync,
    fstrings::*
};

#[cfg(all(test, feature = "cli"))]
use shkeleton::{glob, sherr::fern, dirs};

#[cfg(all(test, feature = "failure"))]
use shkeleton::sherr::failure;

lazy_static! {
    static ref TEST: u64 = { 10 };
}

#[derive(From, Deref)]
struct A(pub u32);

#[cfg(feature = "cli")]
#[test]
fn test_cli_feature() {
    let _ = clap::App::new("test");
    let _mo = glob::MatchOptions::new(); // glob
    let _log_level = fern::Dispatch::new(); // fern
    let _home = dirs::home_dir();
}

#[cfg(feature = "concurrency")]
#[test]
fn test_concurrency_feature() {
    let _num_cpus = num_cpus::get(); // num_cpus
}

#[cfg(feature = "failure")]
#[test]
fn test_failure_feature() {
    let _ = failure::Error::from(sherr::DiagError::unimplemented(diag_position!()));
}

#[test]
fn test_compile() {
    let _chrono = chrono::Utc::now(); // chrono
    let _rw = sync::RwLock::new(10);
    info!("Message {}", *TEST);
    let _regex = regex::Regex::new("[abc]+").unwrap();
    url::Url::parse("http://abc.ru").unwrap();
    let _ = percent_encoding::utf8_percent_encode("abc", percent_encoding::NON_ALPHANUMERIC);
    let mut cur = std::io::Cursor::new(vec![0u8; 8]);
    let _be = cur.read_u64::<byteorder::BigEndian>().unwrap();
    let name = "SomeName";
    let _ = sync::AtomicPtrHolder::<usize>::empty();
    format_f!("My name is {name}");
}

#[test]
#[should_panic]
fn test_diag() {
    let _ = TEST;
    diag_unreachable!()
}
