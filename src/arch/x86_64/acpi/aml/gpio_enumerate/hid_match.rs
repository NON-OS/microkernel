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

use super::sideband::community_pids;

// Intel pinctrl `_HID`s Linux binds a layout to without a sideband entry
// here: Broxton and Gemini Lake declare each community as its own device
// with a static window, and the others are kept for firmware that writes
// static windows (pinctrl-broxton.c, pinctrl-cannonlake.c, pinctrl-icelake.c,
// pinctrl-tigerlake.c, pinctrl-meteorlake.c).
const STATIC_ONLY: [&[u8]; 7] =
    [b"INT3452", b"INT3453", b"INT3450", b"INT34C3", b"INT34C6", b"INTC105E", b"INTC1083"];

// The AMD FCH GPIO bank (Linux pinctrl-amd.c amd_gpio_acpi_match), one
// static window.
const AMD: [&[u8]; 4] = [b"AMD0030", b"AMDI0030", b"AMDI0031", b"AMDI0033"];

/// True for every GPIO controller `_HID` the i2c driver knows a layout for.
pub(super) fn hid_is_gpio_controller(hid: &[u8; 8]) -> bool {
    !community_pids(hid).is_empty()
        || STATIC_ONLY.iter().chain(AMD.iter()).any(|&id| id_matches(hid, id))
}

/// Compare a decoded `_HID` against one identifier, honouring the trailing
/// zero byte a seven-character id leaves in the eight-byte field.
pub(super) fn id_matches(hid: &[u8; 8], id: &[u8]) -> bool {
    if id.len() == 7 {
        &hid[..7] == id && hid[7] == 0
    } else {
        &hid[..8] == id
    }
}
