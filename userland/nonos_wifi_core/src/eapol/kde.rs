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

//! The Key Data field of an EAPOL-Key frame, once unwrapped: a run of
//! information elements and key data encapsulations (KDEs, IEEE Std
//! 802.11-2020, 12.7.2, Table 12-10). Message 3 carries the AP's RSNE (and
//! RSNXE) for the downgrade check, the GTK KDE and, with management frame
//! protection, the IGTK KDE; group message 1 carries the new GTK and IGTK. The
//! walk is bounded, refuses an element that runs past the buffer, and treats
//! the 0xDD-then-zeros tail as the padding the key wrap requires.

/// Element ids found in key data.
const EID_RSN: u8 = 48;
const EID_RSNX: u8 = 244;
const EID_VENDOR: u8 = 0xDD;
/// The IEEE 802.11 OUI that marks an RSN KDE.
const OUI_IEEE: [u8; 3] = [0x00, 0x0F, 0xAC];
/// KDE data types.
const KDE_GTK: u8 = 1;
const KDE_IGTK: u8 = 9;

/// A group temporal key from its KDE: the key index (bits 0-1 of the first
/// octet) and the key bytes.
#[derive(Clone, Copy)]
pub struct GtkKde<'a> {
    pub key_id: u8,
    pub key: &'a [u8],
}

/// An integrity group temporal key from its KDE: the key index (4 or 5), the
/// IGTK packet number the AP last used, and the key bytes.
#[derive(Clone, Copy)]
pub struct IgtkKde<'a> {
    pub key_id: u16,
    pub ipn: [u8; 6],
    pub key: &'a [u8],
}

/// What a Key Data field carried. Elements are kept whole (id and length
/// included) so the RSNE can be compared byte for byte with the beacon's.
#[derive(Clone, Copy, Default)]
pub struct KeyData<'a> {
    pub rsne: Option<&'a [u8]>,
    pub rsnxe: Option<&'a [u8]>,
    pub gtk: Option<GtkKde<'a>>,
    pub igtk: Option<IgtkKde<'a>>,
}

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
                    _ => {}
                }
            }
            _ => {}
        }
        off = end;
    }
    Some(out)
}

// GTK KDE data: key id / Tx octet, a reserved octet, then the key.
fn gtk_kde(d: &[u8]) -> Option<GtkKde<'_>> {
    if d.len() < 3 {
        return None;
    }
    Some(GtkKde { key_id: d[0] & 0x03, key: &d[2..] })
}

// IGTK KDE data: key id (2, little-endian), IPN (6), then the key.
fn igtk_kde(d: &[u8]) -> Option<IgtkKde<'_>> {
    if d.len() < 9 {
        return None;
    }
    let mut ipn = [0u8; 6];
    ipn.copy_from_slice(&d[2..8]);
    Some(IgtkKde { key_id: u16::from_le_bytes([d[0], d[1]]), ipn, key: &d[8..] })
}
