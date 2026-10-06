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

//! Where the fields of a relative mouse report sit, as the descriptor walk
//! found them. Offsets are in bits from the start of the report body.

use super::layout::Field;

/// The fields of a relative mouse report, located by parsing the HID report
/// descriptor. `buttons` spans the button bits, button 1 first.
#[derive(Clone, Copy, Default)]
pub struct MouseLayout {
    pub report_id: u8,
    pub buttons: Field,
    pub button_count: u8,
    pub x: Field,
    pub y: Field,
    pub wheel: Field,
}

impl MouseLayout {
    pub fn is_relative_mouse(&self) -> bool {
        self.x.present() && self.y.present()
    }
}
