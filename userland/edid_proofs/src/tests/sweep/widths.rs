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

//! How wide each field of a timing encoding is, so a generated timing
//! fits the bytes it is written into.

pub struct Widths {
    pub active: u32,
    pub blank: u32,
    pub h_front: u32,
    pub h_sync: u32,
    pub v_front: u32,
    pub v_sync: u32,
    /// The smallest front porch the encoding can say.
    pub front_min: u32,
    pub clock_step_khz: u32,
    pub clock_max_khz: u32,
    /// Whether a zero active or sync can be written: type VII adds one.
    pub can_zero: bool,
}

/// A base block or CTA-861 detailed timing descriptor.
pub const DTD: Widths = Widths {
    active: 4095,
    blank: 4095,
    h_front: 1023,
    h_sync: 1023,
    v_front: 63,
    v_sync: 63,
    front_min: 0,
    clock_step_khz: 10,
    clock_max_khz: 655_350,
    can_zero: true,
};

/// A DisplayID type VII record; type I differs only in its clock unit.
pub const TYPE7: Widths = Widths {
    active: 65_536,
    blank: 65_536,
    h_front: 32_768,
    h_sync: 65_536,
    v_front: 32_768,
    v_sync: 65_536,
    front_min: 1,
    clock_step_khz: 1,
    clock_max_khz: 1 << 24,
    can_zero: false,
};
