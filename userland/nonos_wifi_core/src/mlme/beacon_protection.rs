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

//! Beacon protection, as the station offers it (IEEE Std 802.11-2020: the
//! Extended Capabilities element, bit 84, Beacon Protection Enabled). Only a station that checks its AP's beacons under the BIGTK
//! (`softmac`, since E-45) says so, and only on a link with management
//! frame protection, the only kind a BIGTK comes with.

use super::state::Mlme;
use crate::rsn::{Pmf, Selection};

/// Extended Capabilities, eleven octets, with only bit 84 set: octet 10,
/// bit 4.
pub(in crate::mlme) const EXT_CAPS_BEACON_PROTECTION: [u8; 13] =
    [127, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10];

impl Mlme {
    /// This station checks its AP's beacons under the BIGTK: say so in the
    /// (re)association request. A station that does not call this offers
    /// nothing, and its requests are as before.
    pub fn verify_beacons(&mut self) {
        self.verifies_beacons = true;
    }

    /// The Extended Capabilities element for `sel`, or nothing.
    pub(in crate::mlme) fn ext_caps(&self, sel: &Selection) -> &'static [u8] {
        let offer = self.verifies_beacons && sel.pmf != Pmf::Off;
        if offer {
            &EXT_CAPS_BEACON_PROTECTION
        } else {
            &[]
        }
    }
}
