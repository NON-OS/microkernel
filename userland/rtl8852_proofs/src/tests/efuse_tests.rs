// NONOS Operating System (AGPL-3.0-or-later)
//! The efuse as the probe reads it: the board fields out of the packed
//! physical cells (main map and DAV), the phycap trims, and the chip's
//! PHY capabilities over the mailbox. The programmed MAC address is in the
//! map and must stay there: never read into anything the driver keeps.

use crate::bring::probe::{probe, Probe};
use crate::efuse::map::nibble;
use crate::model::efuse_img::{RFE_TYPE, THERM, XTAL_K};
use crate::model::{rig, Card};

fn probed(card: &Card) -> Probe {
    let mut rings = rig::rings(card);
    let mut fw = crate::h2c::Fw::default();
    probe(card, &mut rings, &mut fw).ok().expect("probe")
}

#[test]
fn the_board_fields_come_out_of_the_packed_cells() {
    let card = Card::new();
    let p = probed(&card);
    let e = &p.efuse.efuse;
    assert!(p.efuse.valid, "autoload done");
    assert_eq!((e.rfe_type, e.xtal_k, e.therm), (RFE_TYPE, XTAL_K, THERM));
    assert_eq!(e.tssi_cck[0], [0x20, 0x21, 0x22, 0x23, 0x24, 0x25]);
    assert_eq!(e.tssi_mcs[1][..5], [0x46, 0x47, 0x48, 0x49, 0x4a]);
    assert_eq!(e.tssi_mcs[1][5], 0x40 + 18, "the 5 GHz groups start at offset 18");
    assert_eq!((e.gain[0][0], e.gain[1][0], e.gain_valid), (1, 2, true));
    assert_eq!(e.gain[0][1], nibble(0xf), "unprogrammed reads -1");
    let c = &p.efuse.phycap;
    assert!(c.power_k_valid);
    assert_eq!(c.thermal_trim, Some([0x05, 0x03]));
    assert_eq!(c.pa_bias_trim, None);
    assert_eq!(p.cv, 1, "cut B");
}

#[test]
fn the_phy_capabilities_come_from_the_mailbox() {
    let card = Card::new();
    card.0.borrow_mut().phy_cap = [0x1_0000, 1, 0, 0];
    let c = probed(&card).caps;
    assert_eq!((c.tx_nss, c.rx_nss, c.rx_b_only), (1, 1, false));
    let s = card.0.borrow();
    assert_eq!(s.mailbox.len(), 1);
    assert_eq!(s.mailbox[0][0] & 0x7f, 3, "GET_FEATURE");
}

#[test]
fn a_capability_of_zero_streams_reads_as_the_chips_two() {
    let card = Card::new();
    card.0.borrow_mut().phy_cap = [0, 0, 0, 0];
    let c = probed(&card).caps;
    assert_eq!((c.tx_nss, c.rx_nss), (2, 2));
}

#[test]
fn the_probe_leaves_the_chip_off() {
    let card = Card::new();
    probed(&card);
    assert!(!card.0.borrow().powered);
}
