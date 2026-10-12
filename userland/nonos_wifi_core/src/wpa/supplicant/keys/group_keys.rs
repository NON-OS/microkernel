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

//! Pull the group keys out of parsed key data.

use super::*;

/// Pull the group keys out of parsed key data. `None` unless a GTK of the
/// group cipher's length `gtk_len` is present, and any IGTK present is BIP-CMAC-128 sized with
/// an index of 4 or 5.
pub(in crate::wpa::supplicant) fn group_keys(
    kd: &KeyData<'_>,
    gtk_len: usize,
) -> Option<GroupKeys> {
    let g = kd.gtk?;
    if g.key.len() != gtk_len || gtk_len > GTK_MAX {
        return None;
    }
    let mut gtk = [0u8; GTK_MAX];
    gtk[..gtk_len].copy_from_slice(g.key);
    let igtk = match kd.igtk {
        Some(i) => {
            if i.key.len() != IGTK_LEN || !(4..=5).contains(&i.key_id) {
                return None;
            }
            let mut key = [0u8; IGTK_LEN];
            key.copy_from_slice(i.key);
            Some((key, i.key_id, i.ipn))
        }
        None => None,
    };
    Some(GroupKeys { gtk, gtk_id: g.key_id, igtk, rsc: 0 })
}

/// The packet number in a Key RSC field (little-endian, six octets used).
pub(in crate::wpa::supplicant) fn rsc_pn(rsc: &[u8; 8]) -> u64 {
    let mut b = [0u8; 8];
    b[..6].copy_from_slice(&rsc[..6]);
    u64::from_le_bytes(b)
}

/// Parse unwrapped key data and extract the group keys from it.
pub(in crate::wpa::supplicant) fn parse_group_keys(
    plain: &[u8],
    gtk_len: usize,
) -> Option<(KeyData<'_>, GroupKeys)> {
    let kd = parse_key_data(plain)?;
    let keys = group_keys(&kd, gtk_len)?;
    Some((kd, keys))
}

/// Overwrite secret bytes so they do not linger on the stack.
pub(in crate::wpa::supplicant) fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: `b` is a valid, aligned, exclusive reference into `buf`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
