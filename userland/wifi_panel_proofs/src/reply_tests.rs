// NONOS Operating System (AGPL-3.0-or-later)
//! The client takes a reply only when it is the answer to its own request:
//! a whole header, inside the buffer, with the request's magic, op and id.

use crate::reply::{answer, header, WIFI_HDR, WIFI_MAGIC};

fn reply(op: u16, rid: u32, body: &[u8]) -> Vec<u8> {
    [&header(op, rid)[..], body].concat()
}

#[test]
fn the_header_is_magic_op_and_request_id() {
    let h = header(3, 0x0102_0304);
    assert_eq!(&h[0..4], &WIFI_MAGIC.to_le_bytes());
    assert_eq!((&h[4..6], &h[6..10]), (&[3, 0][..], &[4, 3, 2, 1][..]));
}

#[test]
fn the_answer_to_this_request_is_taken() {
    let r = reply(1, 7, &[9; 38]);
    assert_eq!(answer(&r, r.len() as i64, 1, 7), Some(r.len()));
    assert_eq!(answer(&r, WIFI_HDR as i64, 1, 7), Some(WIFI_HDR), "a bare header");
}

#[test]
fn any_other_reply_is_refused() {
    let r = reply(1, 7, &[9; 38]);
    let n = r.len() as i64;
    assert_eq!(answer(&r, n, 1, 8), None, "another request's id");
    assert_eq!(answer(&r, n, 2, 7), None, "another op");
    let mut forged = r.clone();
    forged[0] ^= 1;
    assert_eq!(answer(&forged, n, 1, 7), None, "not this family's magic");
    assert_eq!(answer(&r, WIFI_HDR as i64 - 1, 1, 7), None, "shorter than a header");
    assert_eq!(answer(&r, n + 1, 1, 7), None, "longer than the buffer");
    assert_eq!(answer(&r, -110, 1, 7), None, "an error from the call");
    assert_eq!(answer(&r[..4], 4, 1, 7), None, "a buffer smaller than a header");
}
