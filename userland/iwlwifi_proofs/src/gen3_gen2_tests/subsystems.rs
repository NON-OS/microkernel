// NONOS Operating System (AGPL-3.0-or-later)

//! The subsystem quirks. Linux v6.12 pcie/drv.c lists, for each Qu and QuZ
//! CNVi id, subsystem ids whose rows name a config (`IWL_DEV_INFO`), and
//! after them rows by MAC type, step and RF (`_IWL_DEV_INFO`); it searches
//! from the end, so a MAC row that matches wins, and only then a subsystem
//! row, whose Qu-b0 image `iwl_pci_probe` moves to Qu-c0 for hw_rev 0x332
//! and to QuZ for 0x351. The broker's record carries no subsystem id, so the
//! driver chooses by MAC, step and RF alone. Here Linux's choice is rebuilt
//! from those rows for every listed subsystem and an unlisted one, with each
//! MAC, step and HR radio, and the driver's is checked against it: equal
//! wherever Linux gives the MAC its own image; where Linux gives none (a Qu
//! B-step 160 MHz HR2 under an unlisted subsystem) the driver runs Qu-b0;
//! where Linux would load another MAC's image (a QuZ part of a step other
//! than B under a Qu row, or a Qu part under a QuZ row) the driver refuses
//! or runs the MAC's own.

use super::drv_rows::{linux, listed, own, HR1, HR2};
use crate::gen3::select::select;

#[test]
fn the_driver_chooses_what_linux_does_wherever_linux_gives_the_mac_its_own() {
    for device in [0x02F0u16, 0x06F0, 0x34F0, 0x3DF0, 0x4DF0, 0x43F0, 0xA0F0] {
        let subs = listed(device).0.iter().copied().chain([0x1234, 0x0234]);
        for sub in subs {
            for hw_rev in [0x331u32, 0x332, 0x350, 0x351] {
                for rf in [HR1, HR2] {
                    let ours = select(device, hw_rev, u32::from(rf) << 12).ok().map(|r| r.0);
                    let mac = ((hw_rev >> 4) & 0xFFF) as u16;
                    match linux(device, sub, hw_rev, rf) {
                        Some(l) if own(mac, l) => {
                            assert_eq!(ours, Some(l), "{device:#x} {sub:#x} {hw_rev:#x}")
                        }
                        _ => assert!(
                            ours.is_none_or(|i| own(mac, i)),
                            "{device:#x} {sub:#x} {hw_rev:#x}"
                        ),
                    }
                }
            }
        }
    }
}
