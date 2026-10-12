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
//! What a gamepad's reports hold: the input fields this driver reads,
//! each where it sits in its report. Buttons are the Button page's first
//! 32; axes are Generic Desktop X to Wheel (30h to 38h) and the hat switch
//! (39h).

use alloc::vec::Vec;

pub const PAGE_DESKTOP: u16 = 0x01;
pub const PAGE_BUTTON: u16 = 0x09;
pub const JOYSTICK: u16 = 0x04;
pub const GAME_PAD: u16 = 0x05;
pub const HAT: u16 = 0x39;
pub const BUTTONS: u16 = 32;
pub const AXES: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Field {
    pub page: u16,
    pub usage: u16,
    /// The report it is in (0: the device numbers none).
    pub report_id: u8,
    /// From the start of the report's data, after its ID byte.
    pub bit_offset: u32,
    pub bit_size: u32,
    pub min: i32,
    pub max: i32,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct PadLayout {
    /// Whether reports start with an ID byte.
    pub ids: bool,
    pub fields: Vec<Field>,
}

/// Whether `page`:`usage` is a field this driver reads.
pub fn wanted(page: u16, usage: u16) -> bool {
    match page {
        PAGE_BUTTON => (1..=BUTTONS).contains(&usage),
        PAGE_DESKTOP => (0x30..=HAT).contains(&usage),
        _ => false,
    }
}
