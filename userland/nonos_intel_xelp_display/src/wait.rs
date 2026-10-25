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
//! Waiting on a register's bits until a deadline.

use nonos_intel_display_hw::access::{Expiry, Mmio};
use nonos_intel_display_hw::error::Error;

/// Reads `reg` until its `mask` bits equal `want`, or `until` passes. One
/// read is made after the deadline, so a change that lands as the clock
/// runs out still counts.
pub fn bits(
    m: &mut dyn Mmio,
    reg: u32,
    mask: u32,
    want: u32,
    until: &dyn Expiry,
) -> Result<(), Error> {
    loop {
        let done = until.expired();
        if m.read(reg) & mask == want {
            return Ok(());
        }
        if done {
            return Err(Error::Timeout);
        }
    }
}
