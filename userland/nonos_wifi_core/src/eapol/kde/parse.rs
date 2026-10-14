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

//! The bounded walk over a Key Data field.

use super::groups::{gtk_kde, igtk_kde};
use super::kinds::{
    KeyData, EID_RSN, EID_RSNX, EID_VENDOR, KDE_GTK, KDE_IGTK, KDE_TRANSITION_DISABLE, OUI_IEEE,
};

/// Parse unwrapped key data. Returns `None` if an element runs past the end
/// (other than the padding tail) or a GTK or IGTK KDE is malformed, so a
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
        if end > data.len() {
            // 0xDD followed only by zeros is padding, not a truncated KDE.
            if id == EID_VENDOR && data[off + 1..].iter().all(|&b| b == 0) {
                break;
            }
            return None;
        }
        let elem = &data[off..end];
        let content = &data[body..end];
        match id {
            EID_RSN if out.rsne.is_none() => out.rsne = Some(elem),
            EID_RSNX if out.rsnxe.is_none() => out.rsnxe = Some(elem),
            EID_VENDOR if content.len() >= 4 && content[..3] == OUI_IEEE => {
                match content[3] {
                    KDE_GTK if out.gtk.is_none() => out.gtk = Some(gtk_kde(&content[4..])?),
                    KDE_IGTK if out.igtk.is_none() => out.igtk = Some(igtk_kde(&content[4..])?),
                    KDE_TRANSITION_DISABLE if out.transition_disable.is_none() => {
                        out.transition_disable = content.get(4).copied();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        off = end;
    }
    Some(out)
}
