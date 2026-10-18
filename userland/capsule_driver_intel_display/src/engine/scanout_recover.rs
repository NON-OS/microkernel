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

//! Putting a taken-over output back after a fault: the plane is pointed
//! at the front surface again, laid out as the takeover set it, and the
//! call returns once that surface is live. A stopped transcoder is not
//! the plane's to fix; it needs a mode set and is refused untouched.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::flip::wait_live;
use super::pipe::Transcoder;
use super::plane::program;
use super::regs;
use super::scanout::Scanout;

impl Scanout {
    pub fn recover(
        &self,
        m: &mut dyn Mmio,
        t: Transcoder,
        until: &dyn Expiry,
    ) -> Result<(), Error> {
        if m.read(regs::trans_conf(t)) & (1 << 30) == 0 {
            return Err(Error::Stopped);
        }
        let fw = self.firmware;
        let front = self.surfaces[self.front];
        let size = (self.width, self.height);
        program(m, fw.kind, self.pipe, front, self.stride, size, fw.ctl)?;
        wait_live(m, self.pipe, front, until)
    }
}
