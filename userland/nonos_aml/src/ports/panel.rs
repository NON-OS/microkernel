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

//! Which side of the enclosure a _PLD places a device on (ACPI 6.5, 6.1.8,
//! bits 69:67), in the order of the field's values.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Panel {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
    Unknown,
}

pub const PANELS: [Panel; 7] = [
    Panel::Top,
    Panel::Bottom,
    Panel::Left,
    Panel::Right,
    Panel::Front,
    Panel::Back,
    Panel::Unknown,
];
