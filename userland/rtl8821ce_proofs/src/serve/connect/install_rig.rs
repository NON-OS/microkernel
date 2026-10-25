// NONOS Operating System (AGPL-3.0-or-later)

//! The rig of the connect-glue proofs: a join's report, and a link with the
//! AP's next group rekey already waiting on its ring.

use crate::ap_sim::{group1, gtk_kde, GTK};
use crate::assoc::{Outcome, Report};
use crate::link::RtlLink;
use crate::tests_all::linkport_tests::card::Card;
use crate::tests_all::linkport_tests::krack_rig::joined;
use crate::tests_all::linkport_tests::receive::{chip_decrypted, W0_ENC_AES};
use crate::tests_all::linkport_tests::rig::{eth_frame, link_with, Dma, AP_MAC, STA_MAC};
use crate::tests_all::linkport_tests::rxbuf::RxBuf;
use nonos_wifi_core::key::HeldKey;
use nonos_wifi_core::wpa::akm::Akm;

/// The chip's key cache command register (`sec.rs`, RTW_SEC_CMD_REG).
pub(super) const SEC_CMD: usize = 0x0670;

pub(super) fn report(outcome: Outcome) -> Report {
    let (sent, recv, data, eapol, probe, deauth, to_us, state) = (0, 0, 0, 0, 0, 0, 0, 0);
    Report {
        outcome,
        sent,
        recv,
        data,
        eapol,
        probe,
        deauth,
        to_us,
        state,
        failure: None,
        akm: None,
    }
}

// A join's outcome, the AP's next group rekey waiting on the ring.
pub(super) fn joined_with(gtk_id: u8) -> (Report, RtlLink<Card, Dma, RxBuf>, Card) {
    let (sup, ptk, tk) = joined();
    let g1 = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &[0x22; 16]));
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &chip_decrypted(&eth_frame(STA_MAC, AP_MAC, [0x88, 0x8E], &g1), 1), W0_ENC_AES);
    let card = Card::new(0, 1);
    let (link, _tx) = link_with(card.clone(), rx);
    let outcome = Outcome::Joined {
        bssid: AP_MAC,
        channel: 6,
        ptk: HeldKey(tk),
        gtk: HeldKey(GTK),
        gtk_id,
        akm: Akm::Psk,
        supplicant: Some(Box::new(sup)),
    };
    (report(outcome), link, card)
}
