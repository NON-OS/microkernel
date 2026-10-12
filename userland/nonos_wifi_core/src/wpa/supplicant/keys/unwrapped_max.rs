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
//! only at the group cipher's key length and an IGTK only if it is a
//! BIP-CMAC-128 key (16 bytes). The unwrapped plaintext is wiped before return.

use super::*;

/// Bytes of key data the unwrap can hold.
pub(in crate::wpa::supplicant) const UNWRAPPED_MAX: usize = MAX_BLOCKS * 8;

/// The group management cipher's (BIP-CMAC-128) key length.
pub(super) const IGTK_LEN: usize = 16;
/// The longest GTK, GCMP-256's; the group cipher says how much is used.
pub(super) const GTK_MAX: usize = 32;

/// The group keys one message delivered.
pub(in crate::wpa::supplicant) struct GroupKeys {
    pub(in crate::wpa::supplicant) gtk: [u8; GTK_MAX],
    pub(in crate::wpa::supplicant) gtk_id: u8,
    pub(in crate::wpa::supplicant) igtk: Option<([u8; IGTK_LEN], u16, [u8; 6])>,
    /// The Key RSC of the message that delivered them.
    pub(in crate::wpa::supplicant) rsc: u64,
}

impl Supplicant {
    /// Unwrap `wrapped` under the KEK into `plain`, returning its length.
    pub(in crate::wpa::supplicant) fn unwrap_key_data(
        &self,
        wrapped: &[u8],
        plain: &mut [u8; UNWRAPPED_MAX],
    ) -> Option<usize> {
        if wrapped.len() < 16 || !wrapped.len().is_multiple_of(8) {
            return None;
        }
        aes_unwrap(&self.kek(), wrapped, plain)
    }

    /// Store the group keys a message delivered. Returns whether the GTK is a
    /// different key from the one already held (a repeat is not reinstalled).
    pub(in crate::wpa::supplicant) fn store_group_keys(&mut self, k: &GroupKeys) -> bool {
        let len = self.group.tk_len();
        let changed =
            self.gtk_len != len || self.gtk[..len] != k.gtk[..len] || self.gtk_id != k.gtk_id;
        self.gtk[..len].copy_from_slice(&k.gtk[..len]);
        self.gtk_len = len;
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
