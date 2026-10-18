// NONOS Operating System (AGPL-3.0-or-later)
//! What driver.xhci0 sends back, through the driver's own decoder
//! (xhci/reply.rs): a reply is taken only whole and for its own request, and
//! every length in it is held to the bytes that came. A payload length that
//! ran past the bytes that came used to be taken, and the bytes never sent
//! read as data.

use crate::protocol::E_IO;
use crate::xhci::reply::{data_in, moved, reply};
use crate::xhci::wire::{DATA_AT, HDR_LEN, MAGIC};

/// A reply to request `rid` with status 0 and `data`, in a 4 KiB buffer,
/// and the bytes that came.
fn framed(rid: u32, data: &[u8]) -> (Vec<u8>, i64) {
    let mut r = vec![0u8; DATA_AT];
    r[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    r[12..16].copy_from_slice(&rid.to_le_bytes());
    r[16..20].copy_from_slice(&(4 + data.len() as u32).to_le_bytes());
    r.extend_from_slice(data);
    let n = r.len() as i64;
    r.resize(4096, 0);
    (r, n)
}

#[test]
fn a_reply_is_taken_only_whole_and_for_its_own_request() {
    let (resp, n) = framed(7, &[3, 0, 0, 0, 0xA, 0xB, 0xC]);
    assert_eq!(reply(&resp, n, 7), Ok(7));
    assert_eq!(reply(&resp, n, 8), Err(E_IO), "another request's reply");
    assert_eq!(reply(&resp, n - 1, 7), Err(E_IO), "a byte short of its own length");
    assert_eq!(reply(&resp, -110, 7), Err(E_IO));
    assert_eq!(reply(&resp, 4097, 7), Err(E_IO), "more than the buffer holds");
    let mut failed = resp.clone();
    failed[HDR_LEN..DATA_AT].copy_from_slice(&(-71i32).to_le_bytes());
    assert_eq!(reply(&failed, n, 7), Err(-71), "the controller's status");
    let mut out = [0u8; 8];
    assert_eq!(data_in(&resp, 7, 4, &mut out), Ok(3));
    assert_eq!(&out[..3], &[0xA, 0xB, 0xC]);
    assert_eq!(data_in(&resp, 6, 4, &mut out), Err(E_IO), "a count past the reply");
    assert_eq!(data_in(&resp, 7, 4, &mut out[..2]), Err(E_IO), "a count past the buffer");
    assert_eq!((moved(&resp, 3), moved(&resp, 4)), (Err(E_IO), Ok(3)));
}
