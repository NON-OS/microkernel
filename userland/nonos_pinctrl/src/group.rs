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

/// One pad group: pads `first..first + size` of the controller, numbered
/// from `gpio` in the firmware's pin space, or unreachable from ACPI when
/// `gpio` is None.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Group {
    pub first: u16,
    pub size: u16,
    pub gpio: Option<u16>,
}

/// One community: its pads start at controller pad `first`, and its
/// register window is the controller's `_CRS` window at the community's
/// place in `Layout::communities`.
#[derive(Debug, Clone, Copy)]
pub struct Community {
    pub first: u16,
    pub groups: &'static [Group],
}

/// A controller's communities, named after the platform for log lines.
/// Only a layout derived from a published datasheet belongs in `LAYOUTS`.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub name: &'static str,
    pub communities: &'static [Community],
}
