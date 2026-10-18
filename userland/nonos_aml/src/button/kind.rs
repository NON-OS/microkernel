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

//! The control-method buttons and the lid (ACPI 6.5, 4.8.2.2 and 9.4): a
//! power button, PNP0C0C; a sleep button, PNP0C0E; a lid, PNP0C0D. Each
//! reports through Notify on its device rather than a fixed PM1 bit.

use crate::platform::{DeviceInfo, HardwareId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    Power,
    Sleep,
    Lid,
}

/// Notify on a power or sleep button: it was pressed; on a lid: _LID
/// changed (5.6.6).
pub const STATUS_CHANGE: u8 = 0x80;
/// Notify on any of them: it woke the machine (5.6.6, Device Wake).
pub const DEVICE_WAKE: u8 = 0x02;

pub fn button_of(d: &DeviceInfo) -> Option<Button> {
    let has = |id: &[u8; 7]| {
        let want = HardwareId::Eisa(*id);
        d.hid.as_ref() == Some(&want) || d.cids.contains(&want)
    };
    [(b"PNP0C0C", Button::Power), (b"PNP0C0E", Button::Sleep), (b"PNP0C0D", Button::Lid)]
        .into_iter()
        .find_map(|(id, b)| has(id).then_some(b))
}
