// NONOS Operating System (AGPL-3.0-or-later)

//! An Ethernet frame becomes a plaintext MPDU in a gen3 command.

use super::frames::{ethernet, BSSID, FC_PROTECTED, OUR, RATE_N_FLAGS};
use crate::gen3::tx_cmd;
use nonos_wifi_core::station::{Ccmp, LinkStation};

#[test]
fn transmit_frames_a_plaintext_mpdu_into_a_gen3_command() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    // A hardware station frames only once the chip holds the keys.
    sta.chip_keys(true);
    let eth = ethernet();
    let mpdu = sta.tx_frame(&eth).expect("frame the ethernet payload");

    // The radio encrypts, so the frame handed to the TX command is plaintext.
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    assert_eq!(fc & FC_PROTECTED, 0, "hardware CCMP: the framed MPDU is not encrypted");

    let mut cmd = [0u8; tx_cmd::TX_CMD_GEN3_HDR + 1600];
    let n = tx_cmd::build(&mpdu, RATE_N_FLAGS, &mut cmd).expect("wrap in a gen3 TX command");
    assert_eq!(n, tx_cmd::TX_CMD_GEN3_HDR + mpdu.len(), "command is the header plus the frame");
    assert_eq!(&cmd[0..2], &(mpdu.len() as u16).to_le_bytes(), "length field is the frame length");
    assert_eq!(&cmd[tx_cmd::TX_CMD_GEN3_HDR..n], &mpdu[..], "the frame follows the 28-byte header");
}
