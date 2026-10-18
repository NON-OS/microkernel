// NONOS Operating System (AGPL-3.0-or-later)

//! The NVM reply: antennas, LAR and channels.

use crate::gen3::nvm::{parse as parse_nvm, NVM_CHANNELS};
use nonos_wifi_core::regdom::world;

#[test]
fn the_nvm_reply_gives_antennas_lar_and_channels() {
    let mut v4 = vec![0u8; 468];
    v4[12] = 3;
    v4[16] = 1;
    // No LAR: only channels whose profile says VALID.
    v4[28] = 1; // index 0: channel 1
    v4[28 + 14 * 4] = 1; // index 14: channel 36
    let n = parse_nvm(&v4, true).expect("v4");
    assert_eq!((n.valid_tx_ant, n.valid_rx_ant, n.lar_enabled), (3, 1, false));
    assert_eq!(n.channels, vec![1, 36]);
    v4[20] = 1;
    let lar = parse_nvm(&v4, true).unwrap();
    let allowed: Vec<u8> = NVM_CHANNELS.iter().copied().filter(|&c| world(c).is_some()).collect();
    assert_eq!(lar.channels, allowed, "LAR: the 2.4 and 5 GHz channels the domain allows");
    assert_eq!(lar.channels.len(), 13 + 8 + 12 + 5, "1-13; 36-64; 100-144; 149-165");
    assert!(parse_nvm(&v4[..467], true).is_none(), "not exactly version 4's size");
    assert!(parse_nvm(&v4, false).is_none(), "a v4-sized reply read as v3");
    let mut v3 = vec![0u8; 128];
    v3[24] = 1;
    assert_eq!(parse_nvm(&v3, false).unwrap().channels, vec![1]);
}

#[test]
fn a_channel_the_world_domain_refuses_is_dropped_though_the_nvm_says_valid() {
    let mut v4 = vec![0u8; 468];
    for i in [12usize, 13, 22, 47, 50] {
        v4[28 + i * 4] = 1; // channels 13, 14, 68, 169 and 181
    }
    assert_eq!(parse_nvm(&v4, true).unwrap().channels, vec![13]);
}
