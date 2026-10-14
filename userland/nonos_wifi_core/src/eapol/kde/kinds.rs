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

//! What a Key Data field carries, and the ids that mark it.

/// Element ids found in key data.
pub(super) const EID_RSN: u8 = 48;

pub(super) const EID_RSNX: u8 = 244;

pub(super) const EID_VENDOR: u8 = 0xDD;

/// The IEEE 802.11 OUI that marks an RSN KDE.
pub(super) const OUI_IEEE: [u8; 3] = [0x00, 0x0F, 0xAC];

/// KDE data types.
pub(super) const KDE_GTK: u8 = 1;

pub(super) const KDE_IGTK: u8 = 9;
pub(super) const KDE_TRANSITION_DISABLE: u8 = 0x20;
/// Transition Disable bit 0: the network is WPA3-Personal only from now on.
pub const TD_WPA3_PERSONAL: u8 = 1 << 0;

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
    /// The Transition Disable bitmap; a KDE cut before it is ignored.
    pub transition_disable: Option<u8>,
}
