// NONOS Operating System (AGPL-3.0-or-later)
//! Control answers whose lengths and offsets lie. Each is bounded by the
//! bytes that came, so the bind fails by name or the answer is passed
//! over; nothing panics and no read leaves the reply.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Bind;

use crate::control_tests::bind_with;
use crate::device::words;
use crate::qemu::Tamper;
use crate::rndis::query::{info, station};
use crate::rndis::reply::{answer, Answer};

fn query_with(off: u32, len: u32) -> Tamper {
    Box::new(move |k, mut r| {
        if k == 4 {
            r[16..20].copy_from_slice(&len.to_le_bytes());
            r[20..24].copy_from_slice(&off.to_le_bytes());
        }
        vec![r]
    })
}

#[test]
fn query_offsets_and_lengths_past_the_answer_are_no_address() {
    for (off, len) in [(0xFFFF_FFF8, 6), (16, 0xFFFF_FFFF), (u32::MAX, u32::MAX), (17, 6)] {
        let (_, r) = bind_with(query_with(off, len));
        assert!(matches!(r, Bind::Failed("no permanent address", -5)), "off {off:#x} len {len:#x}");
    }
    let r = words(&[0x8000_0004, 30, 2, 0, 6, 16]);
    assert_eq!(info(&r), None, "the bytes stop at MessageLength's claim");
    assert_eq!(station(&words(&[0x8000_0004, 24, 2, 0])), None);
}

#[test]
fn a_message_length_past_the_bytes_or_under_a_header_is_not_an_answer() {
    let mut r = words(&[0x8000_0002, 53, 1, 0]);
    r.resize(52, 0);
    assert_eq!(answer(&r, 0x8000_0002, 1), Answer::Other);
    assert_eq!(answer(&words(&[0x8000_0002, 15, 1, 0]), 0x8000_0002, 1), Answer::Other);
    assert_eq!(answer(&[2, 0, 0, 0x80, 16], 0x8000_0002, 1), Answer::Other);
    assert_eq!(answer(&words(&[7, 0]), 0x8000_0002, 1), Answer::Indication);
    let done = answer(&words(&[0x8000_0002, 16, 1, 0, 9]), 0x8000_0002, 1);
    assert_eq!(done, Answer::Done { status: 0, len: 16 }, "trailing bytes past MessageLength left");
}

#[test]
fn a_short_initialize_completion_halts_and_a_long_one_is_cut_at_512() {
    let short: Tamper = Box::new(|k, mut r| {
        if k == 2 {
            r.truncate(28);
            r[4..8].copy_from_slice(&28u32.to_le_bytes());
        }
        vec![r]
    });
    assert!(matches!(bind_with(short).1, Bind::Failed("INITIALIZE_CMPLT short", -5)));
    let long: Tamper = Box::new(|_, mut r| {
        r.resize(600, 0xEE);
        r[4..8].copy_from_slice(&600u32.to_le_bytes());
        vec![r]
    });
    let (bus, r) = bind_with(long);
    assert!(matches!(r, Bind::Failed("RNDIS INITIALIZE failed", -110)));
    let calls = bus.0.borrow().calls.clone();
    assert!(calls.iter().all(|c| !matches!(c, Call::In(_, n) if *n > 512)));
}
