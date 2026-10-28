// NONOS Operating System (AGPL-3.0-or-later)
//! The size bounds, each at its edge: a TX ring at most 64 descriptors (the
//! release bitmap is a u64), an RX buffer the 16-bit descriptor can name, an
//! H2C body that fits its slot, a frame that fits behind its WD page, a
//! console line of at most `LINE_MAX`; and management slots, like data
//! ones, come back on their release reports.

use super::ring_tests::{tuned, AP};
use crate::h2c::send::{h2c, SendFail, BODY_MAX};
use crate::model::{bus, Card};
use crate::ring::addrs::{ACH0, RXQ};
use crate::ring::rx::{self, RxRing};
use crate::ring::tx::{self, TxRing};
use crate::status::{Line, LINE_MAX};
use crate::tx::wd_ring::FRAME_MAX;
use nonos_wifi_core::softmac::Radio;

#[test]
fn ring_sizes_stop_at_their_edges() {
    let card = Card::new();
    let mut s = card.0.borrow_mut();
    let mem = |s: &mut crate::model::State, n| bus::alloc(&mut s.bus, n);
    assert!(TxRing::new(mem(&mut s, tx::bytes_for(64)), ACH0, 64).is_some());
    assert!(TxRing::new(mem(&mut s, tx::bytes_for(65)), ACH0, 65).is_none());
    assert!(TxRing::new(mem(&mut s, tx::bytes_for(8) - 1), ACH0, 8).is_none());
    assert!(RxRing::new(mem(&mut s, rx::bytes_for(4, 0xffff)), RXQ, 4, 0xffff).is_some());
    assert!(RxRing::new(mem(&mut s, rx::bytes_for(4, 0x10000)), RXQ, 4, 0x10000).is_none());
}

#[test]
fn an_h2c_body_past_its_slot_is_refused_and_nothing_posted() {
    let (card, mut sta) = tuned();
    let c = &mut sta.chip;
    let n = card.0.borrow().h2c.len();
    let cmd = crate::h2c::cam::CAM_UPD;
    let long = vec![0u8; BODY_MAX + 1];
    let sent = h2c(&c.r, &mut c.rings.ch12, &mut c.fw, cmd, &long, false);
    assert_eq!(sent, Err(SendFail::TooLong));
    assert_eq!(card.0.borrow().h2c.len(), n);
}

#[test]
fn a_frame_past_its_slot_is_refused() {
    let (card, mut sta) = tuned();
    let mut f = super::ring_tests::data_to_ap(sta.mac());
    f.resize(FRAME_MAX, 0);
    assert!(sta.send(&f));
    f.push(0);
    assert!(!sta.send(&f));
    assert_eq!(card.0.borrow().sent.len(), 1);
}

#[test]
fn management_slots_come_back_on_their_reports() {
    let (card, mut sta) = tuned();
    let mut deauth = vec![0xc0, 0, 0, 0];
    deauth.extend_from_slice(&AP);
    deauth.extend_from_slice(&sta.mac());
    deauth.extend_from_slice(&AP);
    deauth.extend_from_slice(&[0, 0, 3, 0]);
    for _ in 0..100 {
        assert!(sta.send(&deauth));
    }
    assert_eq!(card.0.borrow().sent.iter().filter(|f| f.ch == 8).count(), 100);
}

#[test]
fn a_console_line_is_cut_never_split() {
    let mut l = Line::new(&[b'x'; 300]);
    l.dec(u32::MAX);
    let b = l.bytes();
    assert_eq!(b.len(), LINE_MAX);
    assert_eq!(b.last(), Some(&b'\n'));
}
