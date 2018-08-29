#[macro_use]
extern crate shkeleton;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate derive_deref;

use shkeleton::byteorder::ReadBytesExt;
use shkeleton::*;

lazy_static! {
    static ref TEST: u64 = { 10 };
}

#[derive(From, Deref)]
struct A(pub u32);

#[cfg(all(test, feature = "cli"))]
#[test]
fn test_cli_feature() {
    let _ = clap::App::new("test");
    let _mo = glob::MatchOptions::new(); // glob
    let _log_level = fern::Dispatch::new(); // fern
}

#[cfg(all(test, feature = "concurrency"))]
#[test]
fn test_concurrency_feature() {
    let _num_cpus = num_cpus::get(); // num_cpus
}

#[cfg(all(test, feature = "failure"))]
#[test]
fn test_failure_feature() {
    let _ = failure::Error::from(sherr::DiagError::unimplemented(diag_position!()));
}

#[cfg(test)]
#[test]
fn test_compile() {
    let _chrono = chrono::Utc::now(); // chrono
    let _rw = sync::RwLock::new(10);
    info!("Message {}", *TEST);
    let _regex = regex::Regex::new("[abc]+").unwrap();
    url::Url::parse("http://abc.ru").unwrap();
    let mut cur = std::io::Cursor::new(vec![0u8; 8]);
    let _be = cur.read_u64::<byteorder::BigEndian>().unwrap();
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_diag() {
    let _ = TEST;
    diag_unreachable!()
}
