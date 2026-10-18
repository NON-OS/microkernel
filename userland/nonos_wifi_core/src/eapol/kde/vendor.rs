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

//! Vendor KDEs (0xDD): the RSN KDEs under the IEEE OUI, and the Wi-Fi
//! Alliance's Transition Disable KDE. That one tells a station which
//! transition modes this network has left behind, so a station that has
//! once joined it under WPA3 never falls back to the older AKM. Its bits
//! are kept as sent; what to do with them is the caller's.

use super::*;

/// The Wi-Fi Alliance OUI that marks its KDEs.
pub(super) const OUI_WFA: [u8; 3] = [0x50, 0x6F, 0x9A];

/// The BIGTK KDE's data type under the IEEE OUI (12.7.2, Table 12-10).
pub(super) const KDE_BIGTK: u8 = 14;

/// The Transition Disable KDE's data type, and its first octet's bits.
pub(super) const KDE_TRANSITION_DISABLE: u8 = 0x20;
pub const TD_WPA3_PERSONAL: u8 = 0x01;
pub const TD_SAE_PK: u8 = 0x02;
pub const TD_WPA3_ENTERPRISE: u8 = 0x04;
pub const TD_ENHANCED_OPEN: u8 = 0x08;

/// Read one vendor KDE's `content` (OUI, data type, data) into `out`. A
/// second KDE of a kind already read is ignored, as is a Transition
/// Disable KDE with no data octet; a short GTK, IGTK or BIGTK KDE fails the
/// whole field.
pub(super) fn vendor_kde<'a>(out: &mut KeyData<'a>, content: &'a [u8]) -> Option<()> {
    let (Some(oui), Some(&kind), data) = (content.get(..3), content.get(3), content.get(4..))
    else {
        return Some(());
    };
    let data = data.unwrap_or_default();
    match (oui, kind) {
        (o, KDE_GTK) if o == OUI_IEEE && out.gtk.is_none() => out.gtk = Some(gtk_kde(data)?),
        (o, KDE_IGTK) if o == OUI_IEEE && out.igtk.is_none() => out.igtk = Some(igtk_kde(data)?),
        (o, KDE_BIGTK) if o == OUI_IEEE && out.bigtk.is_none() => out.bigtk = Some(igtk_kde(data)?),
        (o, KDE_TRANSITION_DISABLE) if o == OUI_WFA && out.transition_disable.is_none() => {
            out.transition_disable = data.first().copied();
        }
        _ => {}
    }
    Some(())
}
