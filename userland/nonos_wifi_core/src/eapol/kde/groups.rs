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

//! The group key KDEs: the GTK's and the IGTK's, each checked whole.

use super::kinds::{GtkKde, IgtkKde};

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
