// NONOS Operating System (AGPL-3.0-or-later)
//! Power save on the link itself, driven by its own clock and counts: a quiet
//! link whose access point was heard sleeps at the period's end; an access
//! point gone quiet wakes it, as does leaving.

use nonos_wifi_core::netif::LinkPort;

use crate::pcie::Aspm;
use crate::ps::{Ps, PERIOD_MS};
use crate::tests_all::linkport_tests::card::Card;
use crate::tests_all::linkport_tests::rig::{link_with, Dma, AP_MAC, PTK};
use crate::tests_all::linkport_tests::rxbuf::RxBuf;

type Link = crate::link::RtlLink<Card, Dma, RxBuf>;

// A beacon from the access point, FCS on, as the chip hands it up.
fn beacon() -> Vec<u8> {
    let head = [&[0x80, 0, 0, 0][..], &[0xFF; 6], &AP_MAC, &AP_MAC, &[0, 0]].concat();
    [head, vec![0; 12], vec![0, 3], b"net".to_vec(), vec![0; 4]].concat()
}

// The firmware told, one beacon waiting, one period run; joined or not.
fn after_a_period(joined: bool) -> Link {
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &beacon());
    let (mut link, _) = link_with(Card::new(0, 1), rx);
    if joined {
        link.associate(AP_MAC, PTK);
    }
    let mut ps = Ps::new(Aspm::default());
    ps.force_on();
    ps.assume_ready();
    link.set_ps(ps, None);
    link.tick(1_000);
    link.tick(1_000 + PERIOD_MS - 1);
    let _ = link.poll_rx(&mut [0u8; 2048]);
    link.tick(1_000 + PERIOD_MS);
    link
}

fn asleep() -> Link {
    after_a_period(true)
}

#[test]
fn a_quiet_link_whose_ap_was_heard_sleeps_and_one_not_joined_does_not() {
    assert!(asleep().ps().asleep());
    assert!(!after_a_period(false).ps().asleep());
}

#[test]
fn an_access_point_gone_quiet_wakes_it() {
    let mut link = asleep();
    link.tick(1_000 + 2 * PERIOD_MS);
    assert!(link.stats().watch_sends > 0, "the station probed");
    assert!(!link.ps().asleep());
}

#[test]
fn leaving_wakes_it() {
    let mut link = asleep();
    link.left_ps();
    assert!(!link.ps().asleep());
}
