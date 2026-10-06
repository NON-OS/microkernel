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

// Linux pinctrl-intel.h INTEL_GPIO_BASE_*: a group's GPIO base equals its
// first pin (MATCH), starts at zero (ZERO), or the group has no ACPI number
// at all (NOMAP).
pub const MATCH: i32 = 0;
pub const NOMAP: i32 = -1;
pub const ZERO: i32 = -2;

/// One pad group: pins `first..first + size` of the controller, numbered
/// from `gpio` in the firmware's pin space, or unreachable from ACPI when
/// `gpio` is None.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Group {
    pub first: u16,
    pub size: u16,
    pub gpio: Option<u16>,
}

/// A group written as Linux writes it, INTEL_GPP(reg, start, end, gpio_base)
/// without the register index, so a table reads line for line against the
/// Linux source it was taken from.
pub const fn gpp(first: u16, last: u16, gpio_base: i32) -> Group {
    let gpio = match gpio_base {
        MATCH => Some(first),
        ZERO => Some(0),
        NOMAP => None,
        g => Some(g as u16),
    };
    Group { first, size: last - first + 1, gpio }
}

/// One community: its pads start at controller pin `first`, and its
/// register window is the controller's resource number `bar`, which is the
/// community's place in `Layout::communities` (Linux intel_community.barno).
#[derive(Debug, Clone, Copy)]
pub struct Community {
    pub first: u16,
    pub groups: &'static [Group],
}

/// A controller's communities, named after the platform for log lines.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub name: &'static str,
    pub communities: &'static [Community],
}
