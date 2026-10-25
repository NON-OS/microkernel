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

//! Whether two port windows, each a base and a count, share a port. Ranges
//! are half open and widened to u32, so a window ending at 0xFFFF cannot
//! wrap onto port 0. Pure, so the proofs run it on the host.

pub(crate) const fn meets(a: (u16, u16), b: (u16, u16)) -> bool {
    let (a0, b0) = (a.0 as u32, b.0 as u32);
    let (a1, b1) = (a0 + a.1 as u32, b0 + b.1 as u32);
    a.1 != 0 && b.1 != 0 && a0 < b1 && b0 < a1
}
