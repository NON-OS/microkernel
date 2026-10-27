// NONOS Operating System (AGPL-3.0-or-later)

//! The guards at the edges: a grant for another request's token is no
//! grant; 6 GHz is never tuned; the station's address is always locally
//! administered and unicast; a ring index past the ring is not trusted; a
//! transmit-free report of more than 63 tokens is read whole; a remapped
//! register's low sixteen bits all reach the window.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

use super::ring_tests::ring;
use crate::mac::txfree::tokens;
use crate::model::rig::{bring_up, chip, regs, up};

#[test]
fn a_grant_for_another_token_is_no_grant() {
    let (mut r, c) = up();
    c.borrow_mut().faults.stale_grant = true;
    assert!(!r.tune(Band::Ghz2, 6));
    assert_eq!(r.diag()[2], 1, "counted as missed");
}

#[test]
fn six_ghz_is_never_tuned() {
    let (mut r, c) = up();
    let asked = c.borrow().mcu.grants;
    assert!(!r.tune(Band::Ghz6, 5) && !r.tune(Band::Ghz6, 37));
    assert_eq!(c.borrow().mcu.grants, asked, "never asked for");
    assert!(r.tune(Band::Ghz2, 5), "while 2.4 GHz is");
}

#[test]
fn every_address_drawn_is_local_and_unicast() {
    for _ in 0..16 {
        let (radio, _) = bring_up(&chip());
        assert_eq!(radio.unwrap().mac()[0] & 0x3, 0x2);
    }
}

#[test]
fn a_ring_index_past_the_ring_gives_nothing_back() {
    let c = chip();
    let r = regs(&c);
    let mut q = ring(8, 64);
    q.reset(&r);
    (0..6).for_each(|_| assert!(q.push(&[1u8; 8]).is_some()));
    c.borrow_mut().regs.insert(0xd4300 + 12, 9);
    q.reclaim(&r);
    assert!(q.push(&[1u8; 8]).is_none(), "index 9 of 8 is not believed");
}

#[test]
fn a_report_of_more_than_63_tokens_is_read_whole() {
    let mut b = ((8 + 4 * 70) | 6 << 27 | 70u32 << 16).to_le_bytes().to_vec();
    b.extend_from_slice(&[0; 4]);
    (0..70u32).for_each(|t| b.extend_from_slice(&(t << 16).to_le_bytes()));
    let mut freed = Vec::new();
    tokens(&b, |t| freed.push(t));
    assert_eq!(freed, (0..70).collect::<Vec<u16>>());
}

#[test]
fn a_remapped_registers_whole_low_half_reaches_the_window() {
    let c = chip();
    regs(&c).wr(0x18001234, 3);
    assert_eq!(c.borrow().l1.get(&0x18001234), Some(&3));
}
