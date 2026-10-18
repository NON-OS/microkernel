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

//! The running progress counters of one join.

#[derive(Default, Clone, Copy)]
pub struct Counters {
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    /// The first non-EAPOL data frame's frame control (low 16) and length.
    pub probe: u32,
    pub deauth: u32,
    /// Data frames addressed to this station rather than a group.
    pub to_us: u32,
}
