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

//! Why a device's reserved ranges were not admitted.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Empty, wrapping, or not on page boundaries.
    Malformed,
    /// Its physical span lies in no reserved-memory region.
    OutsideReservedMemory,
    /// Its IOVA span meets another reserved range or a granted span.
    Overlap,
}

impl Refusal {
    /// What a refused claim says.
    pub const fn reason(&self) -> &'static [u8] {
        match self {
            Refusal::Malformed => b"a reserved range that is empty, wraps or is not page aligned",
            Refusal::OutsideReservedMemory => {
                b"a reserved range outside the tree's reserved memory"
            }
            Refusal::Overlap => b"a reserved range over another, or over a granted one",
        }
    }
}
