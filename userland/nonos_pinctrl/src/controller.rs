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

use crate::group::Layout;
use crate::tables::LAYOUTS;

/// The register model of a GPIO controller.
#[derive(Debug, Clone, Copy)]
pub enum Controller {
    /// Intel PCH pinctrl: one ACPI device whose `_CRS` lists one window per
    /// community, in the order of the layout.
    Intel(&'static Layout),
    /// The AMD FCH GPIO bank: one window, one dword per pin.
    Amd,
}

// Linux pinctrl-amd.c amd_gpio_acpi_match.
const AMD_IDS: [&[u8]; 4] = [b"AMD0030", b"AMDI0030", b"AMDI0031", b"AMDI0033"];

/// The controller model for a decoded `_HID`: seven-character ids carry a
/// trailing zero byte, eight-character ids fill the field.
pub fn controller(hid: &[u8; 8]) -> Option<Controller> {
    if AMD_IDS.iter().any(|id| matches(hid, id)) {
        return Some(Controller::Amd);
    }
    LAYOUTS
        .iter()
        .find(|(ids, _)| ids.iter().any(|id| matches(hid, id)))
        .map(|(_, layout)| Controller::Intel(layout))
}

fn matches(hid: &[u8; 8], id: &[u8]) -> bool {
    match id.len() {
        7 => &hid[..7] == id && hid[7] == 0,
        8 => &hid[..] == id,
        _ => false,
    }
}
