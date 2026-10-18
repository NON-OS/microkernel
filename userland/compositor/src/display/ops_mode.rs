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

//! The ops that change a mode (section 12): MODES, the list a size is
//! fitted against, and MODE_SET, answered once the new primary exists.

use nonos_gfx_wire::body::modes::Modes;
use nonos_gfx_wire::{ops, Errno};

use super::call::{call, SETUP_MS};

pub fn modes(port: u32, output: u16) -> Result<Modes, Errno> {
    let mut b = [0u8; 4 + 12 * nonos_gfx_wire::body::mode::MAX_MODES];
    let n = call((port, output), ops::MODES, &[], &mut b, SETUP_MS)?;
    Modes::decode(&b[..n])
}

/// MODE_SET to `size`; the driver makes the new primary before it breaks
/// the old, so this waits up to the wire's MODE_SET budget.
pub fn mode_set(port: u32, output: u16, size: (u32, u32)) -> Result<(), Errno> {
    let mut p = [0u8; 12];
    p[..4].copy_from_slice(&size.0.to_le_bytes());
    p[4..8].copy_from_slice(&size.1.to_le_bytes());
    call((port, output), ops::MODE_SET, &p, &mut [], ops::MODE_SET_BUDGET_MS).map(|_| ())
}
