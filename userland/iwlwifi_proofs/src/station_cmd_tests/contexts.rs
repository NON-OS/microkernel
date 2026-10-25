// NONOS Operating System (AGPL-3.0-or-later)
//! The PHY, RLC and MAC contexts.

use crate::gen3::cmds::mac_config_add;
use crate::gen3::station::mac::mac_config;
use crate::gen3::station::phy::{phy_context, rlc_config, ACTION_ADD, ACTION_MODIFY, ACTION_REMOVE};
use super::fixtures::{ADDR, image};

#[test]
fn the_phy_context_names_the_channel_band_and_width() {
    let add = phy_context(ACTION_ADD, 6);
    assert_eq!(add.to_vec(), image(&[0, 0, 0, 0, 1, 0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0], 32));
    let five = phy_context(ACTION_ADD, 149);
    assert_eq!(five.to_vec(), image(&[0, 0, 0, 0, 1, 0, 0, 0, 149, 0, 0, 0, 0, 0, 0, 0], 32));
    let remove = phy_context(ACTION_REMOVE, 6);
    assert_eq!(remove.to_vec(), image(&[0, 0, 0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0], 32));
}

#[test]
fn the_rlc_config_carries_the_receive_chains() {
    // Two antennas: valid 0b11 at bit 1, one idle and one active chain (no
    // diversity outside the CAM scheme).
    assert_eq!(rlc_config(3).to_vec(), image(&[0, 0, 0, 0, 0x06, 0x14, 0, 0], 32));
    // One antenna: one chain of each.
    assert_eq!(rlc_config(1).to_vec(), image(&[0, 0, 0, 0, 0x02, 0x14, 0, 0], 32));
}

#[test]
fn the_mac_context_without_association_is_the_bring_ups_add() {
    assert_eq!(mac_config(ACTION_ADD, ADDR, None), mac_config_add(ADDR));
    let mut want = vec![0u8; 52];
    want[4] = 2;
    want[8] = 5;
    want[12..18].copy_from_slice(&ADDR);
    want[20] = 0x04; // group frames, no longer beacons
    want[32] = 1;
    want[36] = 1; // is_assoc
    want[40..42].copy_from_slice(&0x0123u16.to_le_bytes());
    assert_eq!(mac_config(ACTION_MODIFY, ADDR, Some(0x0123)).to_vec(), want);
    let off = mac_config(ACTION_MODIFY, ADDR, None);
    assert_eq!((off[20], off[36], off[40]), (0x0C, 0, 0), "left: beacons accepted again");
}
