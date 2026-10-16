// NONOS Operating System (AGPL-3.0-or-later)

//! The NVM reply read: antennas, channels, and the rest.

use alloc::vec::Vec;

use super::layout::{CHANNEL_VALID, PROFILE_AT, PROFILE_AT_V3, V3_LEN, V4_LEN};
use super::NVM_CHANNELS;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NvmInfo {
    pub valid_tx_ant: u8,
    pub valid_rx_ant: u8,
    pub lar_enabled: bool,
    /// Usable 2.4 and 5 GHz channel numbers, in NVM order.
    pub channels: Vec<u8>,
}

/// Parse the reply; `v4` is whether the firmware sets the API flag.
pub fn parse(payload: &[u8], v4: bool) -> Option<NvmInfo> {
    let (len, profile_at, width) = if v4 { (V4_LEN, PROFILE_AT, 4) } else { (V3_LEN, PROFILE_AT_V3, 2) };
    if payload.len() != len {
        return None;
    }
    let tx = le32(payload, 12)? as u8;
    let rx = le32(payload, 16)? as u8;
    let lar = le32(payload, 20)? != 0;
    let mut channels = Vec::new();
    for (i, &ch) in NVM_CHANNELS.iter().enumerate() {
        let at = profile_at + i * width;
        let flags = if width == 4 {
            le32(payload, at)?
        } else {
            u16::from_le_bytes(payload.get(at..at + 2)?.try_into().ok()?) as u32
        };
        if lar || flags & CHANNEL_VALID != 0 {
            channels.push(ch);
        }
    }
    Some(NvmInfo { valid_tx_ant: tx, valid_rx_ant: rx, lar_enabled: lar, channels })
}

fn le32(d: &[u8], o: usize) -> Option<u32> {
    let b = d.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
