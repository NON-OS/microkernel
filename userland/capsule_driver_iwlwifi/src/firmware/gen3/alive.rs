// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The ALIVE notification (`UCODE_ALIVE_NTFY`, `iwl_alive_ntf_v3` to `_v6`),
//! read the way Linux v6.12 mvm/fw.c `iwl_alive_fn` does: the version comes
//! from the firmware's notification table (5 and 6 carry the SKU id, which
//! decides whether the platform NVM step runs), older layouts are recognised
//! by their exact size, and a payload shorter than its version's structure
//! is refused. The firmware is only alive if the status is 0xCAFE.

/// `UCODE_ALIVE_NTFY`.
pub const UCODE_ALIVE_NTFY: u8 = 0x01;
/// `IWL_ALIVE_STATUS_OK`.
pub const STATUS_OK: u16 = 0xCAFE;

const V3_LEN: usize = 68;
const V4_LEN: usize = 116;
const V5_LEN: usize = 128;
const V6_LEN: usize = 144;
/// `iwl_lmac_alive` is 48 bytes; the debug pointers start 16 bytes in.
const LMAC_DBG: usize = 16;
const SCD_BASE: usize = LMAC_DBG + 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Alive {
    pub status: u16,
    pub umac_major: u32,
    pub umac_minor: u32,
    /// The LMAC's error log in device SRAM, for a crash report.
    pub lmac_error_table: u32,
    pub umac_error_table: u32,
    pub scd_base: u32,
    /// Zero for a part without a platform NVM step.
    pub sku_id: [u32; 3],
}

impl Alive {
    pub fn ok(&self) -> bool {
        self.status == STATUS_OK
    }
}

/// Parse an ALIVE payload. `notif_ver` is the version the firmware's table
/// gives for `UCODE_ALIVE_NTFY` (`None` when it does not say).
pub fn parse(payload: &[u8], notif_ver: Option<u8>) -> Option<Alive> {
    let ver = notif_ver.unwrap_or(0);
    // Where the UMAC data starts (after one or two LMAC blocks), and whether
    // the SKU id follows it.
    let (umac_at, sku) = if ver >= 5 {
        let need = if ver >= 6 { V6_LEN } else { V5_LEN };
        if payload.len() < need {
            return None;
        }
        (4 + 2 * 48, true)
    } else if payload.len() == V4_LEN {
        (4 + 2 * 48, false)
    } else if payload.len() == V3_LEN {
        (4 + 48, false)
    } else {
        return None;
    };
    let mut sku_id = [0u32; 3];
    if sku {
        let at = umac_at + 16;
        for (i, s) in sku_id.iter_mut().enumerate() {
            *s = le32(payload, at + 4 * i)?;
        }
    }
    Some(Alive {
        status: u16::from_le_bytes([payload[0], payload[1]]),
        umac_major: le32(payload, umac_at)?,
        umac_minor: le32(payload, umac_at + 4)?,
        lmac_error_table: le32(payload, 4 + LMAC_DBG)?,
        // `FW_ADDR_CACHE_CONTROL` (the top two bits) is masked off, as Linux does.
        umac_error_table: le32(payload, umac_at + 8)? & !0xC000_0000,
        scd_base: le32(payload, 4 + SCD_BASE)?,
        sku_id,
    })
}

fn le32(d: &[u8], o: usize) -> Option<u32> {
    let b = d.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
