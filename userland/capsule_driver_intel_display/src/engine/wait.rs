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

//! Waiting for a register to show a value, on a deadline.

use super::access::{Expiry, Mmio};
use super::error::Error;

/// Until `(read(offset) & mask) == want`, or the deadline passes.
pub fn wait_for(
    m: &mut dyn Mmio,
    offset: u32,
    mask: u32,
    want: u32,
    until: &dyn Expiry,
) -> Result<(), Error> {
    loop {
        if m.read(offset) & mask == want {
            return Ok(());
        }
        if until.expired() {
            return Err(Error::Timeout);
        }
    }
}
