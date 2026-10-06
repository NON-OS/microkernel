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

//! The pairwise and group keys in the firmware's key table.
//!
//! `struct iwl_sec_key_cmd` (fw/api/datapath.h), SEC_KEY_CMD_API_S_VER_1, 80
//! bytes: `action` 0, then the union at 4. Adding
//! (SEC_KEY_ADD_CMD_API_S_VER_1): `sta_mask` 4, `key_id` 8, `key_flags` 12,
//! `key` 16..48, `tkip_mic_rx_key` 48, `tkip_mic_tx_key` 56, `rx_seq` 64,
//! `tx_seq` 72. Removing (SEC_KEY_REMOVE_CMD_API_S_VER_1): `sta_mask` 4,
//! `key_id` 8, `key_flags` 12, the rest zero (Linux sends the whole
//! structure either way).
//!
//! Filled as Linux v6.12 mvm/mld-key.c `iwl_mvm_mld_send_key` and
//! `__iwl_mvm_sec_key_del` fill it for a station's CCMP keys: the access
//! point's station bit as the mask (on a station interface every key is the
//! access point's), the key index, `IWL_SEC_KEY_FLAG_CIPHER_CCMP` (2), plus
//! `IWL_SEC_KEY_FLAG_MCAST_KEY` (0x20) for the group key and
//! `IWL_SEC_KEY_FLAG_MFP` (0x40) for the pairwise key when management frame
//! protection was negotiated (`iwl_mvm_get_sec_flags`). The 16 key bytes
//! start the 32-byte key field. The transmit counter starts at 0, a new
//! key's `tx_pn`. The receive counter is left 0: the station's own replay
//! check (`nonos_wifi_core::station`) holds every frame to the counters of
//! its key, whether the firmware or the station decrypted it.

use super::phy::{ACTION_ADD, ACTION_REMOVE};

pub const SEC_KEY_LEN: usize = 80;

const CIPHER_CCMP: u32 = 0x02;
const FLAG_MCAST_KEY: u32 = 0x20;
const FLAG_MFP: u32 = 0x40;

/// Which key, for the flags it carries.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyKind {
    /// The pairwise key; `mfp` when management frame protection is on.
    Pairwise { mfp: bool },
    /// A group key.
    Group,
}

/// `key_flags` for a CCMP key of `kind`.
pub fn key_flags(kind: KeyKind) -> u32 {
    CIPHER_CCMP
        | match kind {
            KeyKind::Pairwise { mfp: true } => FLAG_MFP,
            KeyKind::Pairwise { mfp: false } => 0,
            KeyKind::Group => FLAG_MCAST_KEY,
        }
}

fn header(action: u32, sta_mask: u32, key_id: u8, kind: KeyKind) -> [u8; SEC_KEY_LEN] {
    let mut c = [0u8; SEC_KEY_LEN];
    c[0..4].copy_from_slice(&action.to_le_bytes());
    c[4..8].copy_from_slice(&sta_mask.to_le_bytes());
    c[8..12].copy_from_slice(&u32::from(key_id).to_le_bytes());
    c[12..16].copy_from_slice(&key_flags(kind).to_le_bytes());
    c
}

/// Add a 16-byte CCMP key with index `key_id` for the stations in `sta_mask`.
pub fn add_key(sta_mask: u32, key_id: u8, kind: KeyKind, key: &[u8; 16]) -> [u8; SEC_KEY_LEN] {
    let mut c = header(ACTION_ADD, sta_mask, key_id, kind);
    c[16..32].copy_from_slice(key);
    c
}

/// Remove the key with index `key_id` and these flags.
pub fn remove_key(sta_mask: u32, key_id: u8, kind: KeyKind) -> [u8; SEC_KEY_LEN] {
    header(ACTION_REMOVE, sta_mask, key_id, kind)
}
