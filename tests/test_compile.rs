extern crate shkeleton;

use shkeleton::{
    byteorder::ReadBytesExt,
    derive_deref::Deref,
    derive_more::From,
    lazy_static::lazy_static,
    snafu::Snafu,
    sync,
    log::info,
};

#[cfg(all(test, feature = "cli"))]
use shkeleton::{glob, fern, dirs};

lazy_static! {
    static ref TEST: u64 = { 10 };
}

#[derive(Snafu, Debug)]
enum Error {
    #[snafu(display("Some error happened: {}", msg))]
    SomeError { msg: String }
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

#[test]
fn test_compile() {
    let _chrono = chrono::Utc::now(); // chrono
    let _rw = sync::RwLock::new(10);
    info!("Message {}", *TEST);
    let _regex = regex::Regex::new("[abc]+").unwrap();
    url::Url::parse("http://abc.ru").unwrap();
    let mut cur = std::io::Cursor::new(vec![0u8; 8]);
    let _be = cur.read_u64::<byteorder::BigEndian>().unwrap();
    let _err = Error::SomeError { msg: format!("message") };
}
