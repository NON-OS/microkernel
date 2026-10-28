/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The channel information body (76 bytes) of `MCU_EXT_CMD_SET_RX_PATH`,
//! Linux v6.12 `mt7921_mcu_set_chan_info`: control and centre channel,
//! width, transmit streams and the receive chain mask, the switch reason,
//! the band. The driver sends it once, for 2.4 GHz channel 1 at 20 MHz,
//! the channel definition mac80211 starts a phy on, so the firmware's
//! receive path is set; tuning afterwards goes by ROC grants.

use super::caps::Caps;

pub const LEN: usize = 76;
/// `CMD_CBW_20MHZ`.
const BW_20: u8 = 0;
/// `CH_SWITCH_NORMAL`.
const SWITCH_NORMAL: u8 = 0;

/// The body for channel `ch`, 20 MHz wide, on 2.4 GHz (`band` 0) or 5 GHz
/// (`band` 1), for a card with `caps`.
pub fn body(ch: u8, band: u8, caps: Caps) -> [u8; LEN] {
    let mut b = [0u8; LEN];
    let streams = caps.antennas.count_ones() as u8;
    (b[0], b[1], b[2], b[3], b[4]) = (ch, ch, BW_20, streams, caps.antennas);
    (b[5], b[6], b[10]) = (SWITCH_NORMAL, 0, band);
    b
}
