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

//! Recover the group keys from an authentic message 3 or group message 1:
//! AES-key-unwrap the key data under the KEK, read its KDEs, and accept a GTK
//! only if it is a CCMP-128 key (16 bytes) and an IGTK only if it is a
//! BIP-CMAC-128 key (16 bytes). The unwrapped plaintext is wiped before return.

use super::state::Supplicant;
use crate::ccmp::keywrap::{aes_unwrap, MAX_BLOCKS};
use crate::eapol::kde::{parse_key_data, KeyData};

/// Bytes of key data the unwrap can hold.
pub(super) const UNWRAPPED_MAX: usize = MAX_BLOCKS * 8;
/// The group cipher (CCMP-128) and group management cipher (BIP-CMAC-128) key
/// lengths.
const GROUP_KEY_LEN: usize = 16;

/// The group keys one message delivered.
pub(super) struct GroupKeys {
    pub(super) gtk: [u8; GROUP_KEY_LEN],
    pub(super) gtk_id: u8,
    pub(super) igtk: Option<([u8; GROUP_KEY_LEN], u16, [u8; 6])>,
    /// The Key RSC of the message that delivered them.
    pub(super) rsc: u64,
}

impl Supplicant {
    /// Unwrap `wrapped` under the KEK into `plain`, returning its length.
    pub(super) fn unwrap_key_data(&self, wrapped: &[u8], plain: &mut [u8; UNWRAPPED_MAX]) -> Option<usize> {
        if wrapped.len() < 16 || !wrapped.len().is_multiple_of(8) {
            return None;
        }
        aes_unwrap(&self.kek(), wrapped, plain)
    }

    /// Store the group keys a message delivered. Returns whether the GTK is a
    /// different key from the one already held (a repeat is not reinstalled).
    pub(super) fn store_group_keys(&mut self, k: &GroupKeys) -> bool {
        let changed = self.gtk_len != GROUP_KEY_LEN
            || self.gtk[..GROUP_KEY_LEN] != k.gtk
            || self.gtk_id != k.gtk_id;
        self.gtk[..GROUP_KEY_LEN].copy_from_slice(&k.gtk);
        self.gtk_len = GROUP_KEY_LEN;
        self.gtk_id = k.gtk_id;
        self.gtk_rsc = k.rsc;
        if let Some((key, id, ipn)) = k.igtk {
            self.igtk = key;
            self.igtk_id = id;
            self.igtk_ipn = ipn;
            self.igtk_set = true;
        }
        changed
    }
}

/// Pull the group keys out of parsed key data. `None` unless a GTK of the
/// CCMP-128 length is present, and any IGTK present is BIP-CMAC-128 sized with
/// an index of 4 or 5.
pub(super) fn group_keys(kd: &KeyData<'_>) -> Option<GroupKeys> {
    let g = kd.gtk?;
    if g.key.len() != GROUP_KEY_LEN {
        return None;
    }
    let mut gtk = [0u8; GROUP_KEY_LEN];
    gtk.copy_from_slice(g.key);
    let igtk = match kd.igtk {
        Some(i) => {
            if i.key.len() != GROUP_KEY_LEN || !(4..=5).contains(&i.key_id) {
                return None;
            }
            let mut key = [0u8; GROUP_KEY_LEN];
            key.copy_from_slice(i.key);
            Some((key, i.key_id, i.ipn))
        }
        None => None,
    };
    Some(GroupKeys { gtk, gtk_id: g.key_id, igtk, rsc: 0 })
}

/// The packet number in a Key RSC field (little-endian, six octets used).
pub(super) fn rsc_pn(rsc: &[u8; 8]) -> u64 {
    let mut b = [0u8; 8];
    b[..6].copy_from_slice(&rsc[..6]);
    u64::from_le_bytes(b)
}

/// Parse unwrapped key data and extract the group keys from it.
pub(super) fn parse_group_keys(plain: &[u8]) -> Option<(KeyData<'_>, GroupKeys)> {
    let kd = parse_key_data(plain)?;
    let keys = group_keys(&kd)?;
    Some((kd, keys))
}

/// Overwrite secret bytes so they do not linger on the stack.
pub(super) fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: `b` is a valid, aligned, exclusive reference into `buf`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
