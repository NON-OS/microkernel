// NONOS Operating System (AGPL-3.0-or-later)

//! The NVM reply: antennas, LAR and channels.

use crate::gen3::nvm::{parse as parse_nvm, NVM_CHANNELS};

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
    assert_eq!(lar.channels, NVM_CHANNELS.to_vec(), "LAR: all 2.4 and 5 GHz channels");
    assert!(parse_nvm(&v4[..467], true).is_none(), "not exactly version 4's size");
    assert!(parse_nvm(&v4, false).is_none(), "a v4-sized reply read as v3");
    let mut v3 = vec![0u8; 128];
    v3[24] = 1;
    assert_eq!(parse_nvm(&v3, false).unwrap().channels, vec![1]);
}
