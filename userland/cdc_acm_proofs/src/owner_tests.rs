// NONOS Operating System (AGPL-3.0-or-later)
//! One holder at a time, a dead one let go; the read buffer holds what
//! fits whole; and the NACM framing.

use crate::acm::{Owner, Rx};
use crate::frame::{decode, reply};
use crate::wire::{HDR_LEN, MAGIC, OP_READ};

#[test]
fn one_holder_and_a_dead_one_does_not_keep_the_line() {
    let mut o = Owner::default();
    assert_eq!(o.open(7, |_| true), Ok(()));
    assert_eq!(o.open(8, |_| true), Err(-16));
    assert_eq!(o.own(8), Err(-1));
    assert_eq!(o.open(7, |_| true), Ok(()), "the holder again");
    assert_eq!(o.open(8, |p| p != 7), Ok(()), "7 is gone");
    assert_eq!((o.own(8), o.own(7)), (Ok(()), Err(-1)));
    o.close();
    assert_eq!(o.own(8), Err(-1));
}

#[test]
fn reads_are_armed_only_while_they_fit_whole() {
    let mut rx = Rx::default();
    assert!(rx.fits(4096));
    rx.put(&vec![1u8; 13 * 1024]);
    assert!(!rx.fits(4096) && rx.fits(3 * 1024));
    rx.put(&vec![2u8; 8 * 1024]);
    assert_eq!(rx.queued(), 16 * 1024, "kept at the cap");
    assert_eq!(rx.take(4), vec![1u8; 4]);
    rx.clear();
    assert_eq!(rx.take(10), Vec::<u8>::new());
}

#[test]
fn a_request_reads_and_a_reply_is_framed() {
    let mut msg = vec![0u8; HDR_LEN + 2];
    msg[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    msg[4..6].copy_from_slice(&1u16.to_le_bytes());
    msg[6..8].copy_from_slice(&OP_READ.to_le_bytes());
    msg[12..16].copy_from_slice(&9u32.to_le_bytes());
    msg[16..20].copy_from_slice(&2u32.to_le_bytes());
    let (req, body) = decode(&msg).expect("a request");
    assert_eq!((req.op, req.id, body.len()), (OP_READ, 9, 2));
    let mut tx = [0u8; 64];
    let n = reply(&mut tx, &req, -22, b"ab");
    assert_eq!(
        (n, &tx[16..20], &tx[20..24], &tx[24..26]),
        (26, &[6, 0, 0, 0][..], &(-22i32).to_le_bytes()[..], &b"ab"[..])
    );
    assert!(decode(&msg[..HDR_LEN + 1]).is_none(), "body cut short");
}
