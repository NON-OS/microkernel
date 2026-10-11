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

/// An interrupt this CPU has acknowledged and not yet finished.
///
/// `raw` is what the acknowledge register returned and what the end of
/// interrupt has to write back. On GICv3 that is the INTID itself; GICv2 adds
/// the sending CPU to an SGI's value and refuses an EOI that leaves it out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Acked {
    pub intid: u32,
    pub(super) raw: u32,
}
