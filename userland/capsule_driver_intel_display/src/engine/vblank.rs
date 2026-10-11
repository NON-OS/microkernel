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

//! Waiting for a vertical blank: PIPE_FRMCNT steps at each one's start
//! (SKL PRM Vol 2c, PIPE_FRMCNT), so a change in it is a blank passed.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::pipe::Pipe;
use super::regs;

pub fn wait_vblank(m: &mut dyn Mmio, p: Pipe, until: &dyn Expiry) -> Result<(), Error> {
    let start = m.read(regs::pipe_frmcnt(p));
    loop {
        if m.read(regs::pipe_frmcnt(p)) != start {
            return Ok(());
        }
        if until.expired() {
            return Err(Error::Timeout);
        }
    }
}
