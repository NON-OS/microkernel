// NONOS Operating System (AGPL-3.0-or-later)
//! The link from the chip's link word: read one tick in LOOK_EVERY, up
//! only once receive is on, the gap set for the speed, down at once with
//! receive gated and the MAC reset, and a failed read said.

use nonos_usbnet::Nic;

use crate::chip::{Access, PLA};
use crate::helpers::{bound, last, link_to, up, wrote, GIGA_UP};
use crate::rtl::watch::LOOK_EVERY;

#[test]
fn the_link_reads_down_from_bind_and_follows_the_word() {
    let (p, mut r) = bound(0x5c30);
    assert!(!r.link_up());
    p.chip.borrow_mut().link = GIGA_UP;
    for _ in 1..LOOK_EVERY {
        r.tick().unwrap();
        assert!(!r.link_up(), "looked before its turn");
    }
    r.tick().unwrap();
    assert!(r.link_up());
    link_to(&p, &mut r, 0).unwrap();
    assert!(!r.link_up());
    let down =
        last(&p, |a| matches!(*a, Access::Write(PLA, 0xc010, _, v) if v & 0x0f == 0)).unwrap();
    let reset = last(&p, wrote(PLA, 0xe810, 0x1000_0000)).unwrap();
    assert!(down < reset);
    link_to(&p, &mut r, GIGA_UP).unwrap();
    assert!(r.link_up(), "the link came back");
}

#[test]
fn a_link_up_at_bind_reads_up_at_once() {
    let p = crate::chip::plug(0x6010);
    p.chip.borrow_mut().link = GIGA_UP;
    let nonos_usbnet::Bind::Ours(r) = crate::helpers::try_bind(&p) else { panic!("not bound") };
    assert!(r.link_up());
}

#[test]
fn half_duplex_at_100_takes_the_long_gap_and_full_duplex_the_short() {
    for (word, gap) in [(0x08 | 0x02, 0x0200), (0x08 | 0x02 | 0x01, 0x0300), (GIGA_UP, 0x0300)] {
        let (p, mut r) = bound(0x5c10);
        link_to(&p, &mut r, word).unwrap();
        assert_eq!(p.chip.borrow().stored(PLA, 0xe610) >> 16 & 0x0308, gap, "{word:02x}");
    }
}

#[test]
fn a_link_word_that_cannot_be_read_fails_the_tick() {
    let (p, mut r) = up(0x5c00);
    let n = p.chip.borrow().transfers;
    p.chip.borrow_mut().fail_after = Some(n);
    assert_eq!(link_to(&p, &mut r, 0), Err(-71));
    assert!(r.link_up(), "a read that failed changed the link");
}
