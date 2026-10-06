// NONOS Operating System (AGPL-3.0-or-later)
//! Which ports the walk brings up and which it serves, by PxSIG. The values
//! are the ones a live q35 boot showed on both controllers right after the
//! HBA reset (PxSIG 0xFFFFFFFF, PxTFD 0x7F, PxSSTS 0x113) and the ones QEMU
//! posts once FIS receive is on.

use crate::engine::link::disk_sig::{is_ata_disk, may_be_disk};
use crate::engine::link::established::link_established;
use crate::engine::link::ready::device_ready;

#[test]
fn a_port_fresh_from_hba_reset_is_still_tried() {
    assert!(link_established(0x113));
    assert!(may_be_disk(0xFFFF_FFFF));
    assert!(!is_ata_disk(0xFFFF_FFFF));
    /*
     * The reset task file has DRQ set: link_up must see the device's D2H FIS,
     * which needs FRE, before it counts the device ready.
     */
    assert!(!device_ready(0x7F));
}

#[test]
fn a_known_other_kind_is_not_tried() {
    assert!(!may_be_disk(0xEB14_0101));
    assert!(!may_be_disk(0xC33C_0101));
    assert!(!may_be_disk(0x9669_0101));
}

#[test]
fn only_an_ata_signature_is_served() {
    assert!(may_be_disk(0x0000_0101));
    assert!(is_ata_disk(0x0000_0101));
    assert!(!is_ata_disk(0xEB14_0101));
    assert!(!is_ata_disk(0));
}
