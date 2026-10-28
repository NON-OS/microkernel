/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What the firmware says the card has, from `MCU_CE_CMD_GET_NIC_CAPAB`'s
//! reply as Linux v6.12 `mt7921_mcu_get_nic_capability` reads it: a count,
//! then `(type, len)` records. Only the PHY record (`MT_NIC_CAP_PHY`, type
//! 8, `struct mt7921_phy_cap`) is taken: its stream count and which bands
//! the RF paths serve. The factory address record (type 7) is skipped
//! unread: the station never transmits from it.

const CAP_PHY: u32 = 8;
const PHY_NSS: usize = 4;
const PHY_HW_PATH: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Caps {
    pub ghz2: bool,
    pub ghz5: bool,
    /// `antenna_mask`: one bit per spatial stream.
    pub antennas: u8,
}

/// Every MT7921-family card is dual band, two by two: what the driver
/// assumes when the reply says nothing of its PHY.
pub const DEFAULT: Caps = Caps { ghz2: true, ghz5: true, antennas: 0x3 };

fn le32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// The capabilities in reply body `b`, or `DEFAULT` where it says nothing.
pub fn parse(b: &[u8]) -> Caps {
    let n = b.get(..2).map_or(0, |c| u16::from_le_bytes([c[0], c[1]]));
    let mut at = 4usize;
    for _ in 0..n {
        let (Some(kind), Some(len)) = (le32(b, at), le32(b, at + 4)) else { break };
        let body = at + 8;
        let Some(end) = body.checked_add(len as usize).filter(|&e| e <= b.len()) else { break };
        if kind == CAP_PHY && len as usize > PHY_HW_PATH {
            let nss = b[body + PHY_NSS].clamp(1, 4);
            let path = b[body + PHY_HW_PATH];
            return Caps { ghz2: path & 1 != 0, ghz5: path & 2 != 0, antennas: (1 << nss) - 1 };
        }
        at = end;
    }
    DEFAULT
}
