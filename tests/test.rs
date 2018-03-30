extern crate shkeleton;
#[macro_use]
extern crate log;

use shkeleton::*;
use shkeleton::byteorder::ReadBytesExt;

#[test]
fn test_compile() {
    let _rw = sync::RwLock::new(10);
    info!("Message");
    let _regex = regex::Regex::new("[abc]+").unwrap();
    url::Url::parse("http://abc.ru").unwrap();
    let mut cur = std::io::Cursor::new(vec![0u8; 8]);
    let _be = cur.read_u64::<byteorder::BigEndian>().unwrap();
}