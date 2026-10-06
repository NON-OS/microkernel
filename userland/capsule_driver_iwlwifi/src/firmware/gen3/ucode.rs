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

//! What the driver needs from a gen3 firmware file beyond its code: the image
//! loader, the capability and API bitmaps, the per-command version table, the
//! PHY configuration (valid antennas) and the scan channel limit. Parsed the
//! way Linux v6.12 iwl-drv.c `iwl_parse_tlv_firmware` does: the 88-byte
//! header with magic 0x0A4C5749, then `[type u32][len u32][payload]` records
//! padded to four bytes. Every length is checked against the file; a record
//! running past the end or a fixed-size record of the wrong size makes the
//! whole file refused rather than half-read, and, as in Linux, a bitmap word
//! past the known range is ignored and a command table's partial last entry
//! dropped.

use super::find_iml;

const HEADER_LEN: usize = 88;
const MAGIC: u32 = 0x0A4C_5749;
const TLV_PHY_SKU: u32 = 23;
const TLV_API_CHANGES_SET: u32 = 29;
const TLV_ENABLED_CAPABILITIES: u32 = 30;
const TLV_N_SCAN_CHANNELS: u32 = 31;
const TLV_CMD_VERSIONS: u32 = 48;
/// `NUM_IWL_UCODE_TLV_CAPA` and `NUM_IWL_UCODE_TLV_API`: 128 bits each.
const BITMAP_WORDS: usize = 4;
/// A version the table marks as not reported (`IWL_FW_CMD_VER_UNKNOWN`).
const VER_UNKNOWN: u8 = 99;
/// Legacy commands sent with the wide header are listed under `LONG_GROUP`.
const LONG_GROUP: u8 = 1;

/// The parsed firmware description. The sections themselves are classified
/// separately (`dram_map::classify`).
pub struct Ucode<'a> {
    pub iml: &'a [u8],
    pub phy_config: u32,
    pub n_scan_channels: u32,
    capa: [u32; BITMAP_WORDS],
    api: [u32; BITMAP_WORDS],
    cmd_versions: &'a [u8],
}

impl<'a> Ucode<'a> {
    /// Parse `blob`, or `None` if it is not a well-formed iwlwifi TLV file or
    /// carries no image loader (every gen3 image has one).
    pub fn parse(blob: &'a [u8]) -> Option<Self> {
        if blob.len() < HEADER_LEN || le32(blob, 0)? != 0 || le32(blob, 4)? != MAGIC {
            return None;
        }
        let mut u = Ucode {
            iml: &[],
            phy_config: 0,
            n_scan_channels: 0,
            capa: [0; BITMAP_WORDS],
            api: [0; BITMAP_WORDS],
            cmd_versions: &[],
        };
        let mut off = HEADER_LEN;
        while off < blob.len() {
            let ty = le32(blob, off)?;
            let len = le32(blob, off.checked_add(4)?)? as usize;
            let body = off.checked_add(8)?;
            let data = blob.get(body..body.checked_add(len)?)?;
            match ty {
                TLV_PHY_SKU if len == 4 => u.phy_config = le32(data, 0)?,
                TLV_N_SCAN_CHANNELS if len == 4 => u.n_scan_channels = le32(data, 0)?,
                TLV_ENABLED_CAPABILITIES if len == 8 => set_word(&mut u.capa, data)?,
                TLV_API_CHANGES_SET if len == 8 => set_word(&mut u.api, data)?,
                TLV_PHY_SKU | TLV_N_SCAN_CHANNELS | TLV_ENABLED_CAPABILITIES | TLV_API_CHANGES_SET => {
                    return None
                }
                TLV_CMD_VERSIONS => u.cmd_versions = data,
                _ => {}
            }
            // Records are padded to four bytes; the last may end the file.
            off = body.checked_add((len + 3) & !3)?;
        }
        // The image loader, found by the same walk the section split uses.
        u.iml = find_iml(blob).unwrap_or(&[]);
        (!u.iml.is_empty()).then_some(u)
    }

    /// Whether capability `bit` (`IWL_UCODE_TLV_CAPA_*`) is set.
    pub fn capa(&self, bit: u32) -> bool {
        test(&self.capa, bit)
    }

    /// Whether API flag `bit` (`IWL_UCODE_TLV_API_*`) is set.
    pub fn api(&self, bit: u32) -> bool {
        test(&self.api, bit)
    }

    /// The command version the firmware reports for `cmd` in `group`, or
    /// `None` when it does not say (Linux then uses the caller's default).
    /// Legacy commands (group 0) are listed under `LONG_GROUP`, as
    /// `iwl_fw_lookup_cmd_ver` maps them.
    pub fn cmd_version(&self, group: u8, cmd: u8) -> Option<u8> {
        let group = if group == 0 { LONG_GROUP } else { group };
        self.lookup(group, cmd).map(|e| e[2]).filter(|&v| v != VER_UNKNOWN)
    }

    /// The notification version for `cmd` in `group`, looked up in exactly
    /// that group (`iwl_fw_lookup_notif_ver` does not remap).
    pub fn notif_version(&self, group: u8, cmd: u8) -> Option<u8> {
        self.lookup(group, cmd).map(|e| e[3]).filter(|&v| v != VER_UNKNOWN)
    }

    /// Valid transmit antennas (`FW_PHY_CFG_TX_CHAIN`, bits 16-19).
    pub fn valid_tx_ant(&self) -> u8 {
        ((self.phy_config >> 16) & 0xF) as u8
    }

    /// Valid receive antennas (`FW_PHY_CFG_RX_CHAIN`, bits 20-23).
    pub fn valid_rx_ant(&self) -> u8 {
        ((self.phy_config >> 20) & 0xF) as u8
    }

    // The table entry `[cmd][group][cmd_ver][notif_ver]`.
    fn lookup(&self, group: u8, cmd: u8) -> Option<&'a [u8]> {
        self.cmd_versions.chunks_exact(4).find(|e| e[0] == cmd && e[1] == group)
    }
}

// A capability or API record: `[word index u32][bits u32]`. A word past the
// 128 bits this driver knows is ignored.
fn set_word(words: &mut [u32; BITMAP_WORDS], data: &[u8]) -> Option<()> {
    let index = le32(data, 0)? as usize;
    let bits = le32(data, 4)?;
    if let Some(w) = words.get_mut(index) {
        *w = bits;
    }
    Some(())
}

fn test(words: &[u32; BITMAP_WORDS], bit: u32) -> bool {
    words.get((bit / 32) as usize).is_some_and(|w| w & (1 << (bit % 32)) != 0)
}

fn le32(d: &[u8], o: usize) -> Option<u32> {
    let b = d.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
