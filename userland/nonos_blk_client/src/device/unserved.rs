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

//! How many controllers on the bus have no service behind them. Pure, with
//! no imports, so the host proofs mount it.

/// When each controller is a disk of its own (`one_disk_each`), every one
/// past those served is a disk missing. Otherwise a driver serves its disk
/// from whichever controller holds one, so only a driver serving nothing,
/// with a controller on the bus, is a disk missing.
pub fn unserved(one_disk_each: bool, on_bus: usize, served: u8) -> u8 {
    let missing = if one_disk_each {
        on_bus.saturating_sub(served as usize)
    } else {
        usize::from(served == 0 && on_bus > 0)
    };
    missing.min(u8::MAX as usize) as u8
}
