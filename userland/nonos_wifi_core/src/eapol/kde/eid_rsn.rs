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
pub(super) const EID_RSN: u8 = 48;

pub(super) const EID_RSNX: u8 = 244;

pub(super) const EID_VENDOR: u8 = 0xDD;

/// The IEEE 802.11 OUI that marks an RSN KDE.
pub(super) const OUI_IEEE: [u8; 3] = [0x00, 0x0F, 0xAC];

/// KDE data types.
pub(super) const KDE_GTK: u8 = 1;

pub(super) const KDE_IGTK: u8 = 9;

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
