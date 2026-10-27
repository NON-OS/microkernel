// NONOS Operating System (AGPL-3.0-or-later)

//! What the real chip may deliver that the driver must not lose: a channel
//! grant on the data ring (Linux takes `PKT_TYPE_RX_EVENT` from any ring),
//! a grant that names the channel but not the token (Linux reads neither),
//! and receive descriptors of the later layout, dropped only for the
//! reasons `mt76_dma_should_drop_buf` names.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

use crate::dma::desc::rx_done;
use crate::model::rig::up;

#[test]
fn a_grant_on_the_data_ring_tunes_the_radio() {
    let (mut r, c) = up();
    c.borrow_mut().faults.grant_on_data = true;
    assert!(r.tune(Band::Ghz2, 6) && r.tune(Band::Ghz5, 149));
    assert_eq!((r.diag()[1], r.diag()[2]), (2, 0), "two grants, none missed");
}

#[test]
fn a_grant_naming_the_channel_without_the_token_is_taken() {
    let (mut r, c) = up();
    c.borrow_mut().faults.tokenless_grant = true;
    assert!(r.tune(Band::Ghz2, 11));
    assert_eq!(c.borrow().mcu.roc, Some(11));
}

#[test]
fn the_later_layout_drops_only_for_its_reasons() {
    let done = 1 << 31 | 1 << 30 | 100 << 16;
    let ver = done | 1 << 7;
    assert_eq!(rx_done(done, 0, 0), (100, false, false));
    assert!(rx_done(done | 1 << 12, 0, 0).2 && rx_done(done | 1 << 14, 0, 0).2, "TO_HOST_A, DROP");
    assert!(!rx_done(ver, 0, 0).2, "the later layout alone is no reason");
    assert!(rx_done(ver, 1 << 12, 0).2, "a repeat");
    assert!(rx_done(ver, 2 << 12, 0).2 && !rx_done(ver, 2 << 12, 1 << 9).2, "an old frame");
    assert!(rx_done(ver | 1 << 13, 0, 0).2, "a PN check failed");
}
