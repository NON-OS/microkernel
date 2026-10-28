// NONOS Operating System (AGPL-3.0-or-later)
//! A request cut short anywhere in its header is refused, never read past
//! its end: any process that can send to the driver can send 6 bytes.

use crate::wire::decode;
use crate::wire::{HDR_LEN, MAGIC, VERSION};

fn whole() -> Vec<u8> {
    let mut msg = vec![0u8; HDR_LEN];
    msg[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    msg[4..6].copy_from_slice(&VERSION.to_le_bytes());
    msg[6..8].copy_from_slice(&3u16.to_le_bytes());
    msg[12..16].copy_from_slice(&5u32.to_le_bytes());
    msg
}

#[test]
fn every_cut_of_the_header_is_refused() {
    let msg = whole();
    let (req, body) = decode(&msg).expect("the whole header is a request");
    assert_eq!((req.op, req.id, body.len()), (3, 5, 0));
    for n in 0..HDR_LEN {
        assert!(decode(&msg[..n]).is_none(), "{n} bytes read as a request");
    }
}
