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

//! Parse unwrapped key data.

use super::*;

/// Parse unwrapped key data. Returns `None` if an element runs past the end
/// or a GTK or IGTK KDE is malformed, so a
/// truncated or forged field is never half-read. Unknown elements and vendor
/// KDEs are skipped; the first of a repeated element is kept.
pub fn parse_key_data(data: &[u8]) -> Option<KeyData<'_>> {
    let mut out = KeyData::default();
    let mut off = 0usize;
    while off + 2 <= data.len() {
        let id = data[off];
        let len = data[off + 1] as usize;
        let body = off + 2;
        let end = body.checked_add(len)?;
        // The key wrap's padding, 0xDD then zeros, reads as elements of
        // length zero, so it never runs past the field.
        if end > data.len() {
            return None;
        }
        let elem = &data[off..end];
        let content = &data[body..end];
        match id {
            EID_RSN if out.rsne.is_none() => out.rsne = Some(elem),
            EID_RSNX if out.rsnxe.is_none() => out.rsnxe = Some(elem),
            EID_VENDOR => vendor_kde(&mut out, content)?,
            _ => {}
        }
        off = end;
    }
    Some(out)
}

// GTK KDE data: key id / Tx octet, a reserved octet, then the key.
pub(super) fn gtk_kde(d: &[u8]) -> Option<GtkKde<'_>> {
    if d.len() < 3 {
        return None;
    }
    Some(GtkKde { key_id: d[0] & 0x03, key: &d[2..] })
}

// IGTK KDE data: key id (2, little-endian), IPN (6), then the key.
pub(super) fn igtk_kde(d: &[u8]) -> Option<IgtkKde<'_>> {
    if d.len() < 9 {
        return None;
    }
    let mut ipn = [0u8; 6];
    ipn.copy_from_slice(&d[2..8]);
    Some(IgtkKde { key_id: u16::from_le_bytes([d[0], d[1]]), ipn, key: &d[8..] })
}
