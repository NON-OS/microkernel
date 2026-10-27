// NONOS Operating System (AGPL-3.0-or-later)
//! The rings under load and under hostile input: a frame slot is reused
//! only after its release report (`rtw89_pci_release_rpp`); a full ring
//! refuses and counts.

use crate::chan::Chan;
use crate::model::rig;
use crate::radio::Station;
use nonos_wifi_core::softmac::Radio;

pub(super) const AP: [u8; 6] = [2, 0, 0, 0, 0, 0xaa];

pub(super) fn data_to_ap(sta: [u8; 6]) -> Vec<u8> {
    let mut f = vec![0x08, 0x01, 0, 0];
    f.extend_from_slice(&AP);
    f.extend_from_slice(&sta);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&[0, 0, 0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45]);
    f
}

pub(super) fn tuned() -> (crate::model::Card, Station<crate::model::Card, crate::model::bus::Mem>) {
    let (card, mut chip) = rig::ready();
    chip.tune_to(Chan::new(false, 6).unwrap()).unwrap();
    (card, Station::new(chip))
}

#[test]
fn released_slots_are_reused_without_end() {
    let (card, mut sta) = tuned();
    let f = data_to_ap(sta.mac());
    for _ in 0..200 {
        assert!(sta.send(&f));
    }
    assert_eq!(card.0.borrow().sent.len(), 200);
    assert_eq!(sta.counts.tx_full, 0);
}

#[test]
fn without_release_reports_the_ring_fills_and_refuses() {
    let (card, mut sta) = tuned();
    card.0.borrow_mut().faults.no_release = true;
    let f = data_to_ap(sta.mac());
    let taken = (0..40).take_while(|_| sta.send(&f)).count();
    assert_eq!(taken, 32, "every slot taken, none before its report");
    assert!(!sta.send(&f));
    assert_eq!(sta.counts.tx_full, 2);
    assert_eq!(card.0.borrow().sent.len(), 32, "no slot overwritten before release");
}
