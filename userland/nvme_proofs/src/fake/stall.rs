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

//! An I/O command left stuck on the fake, and the driver's recovery from it.

use super::Up;
use crate::error::NvmeError;
use crate::recover::{recover, Recovered, Stalled};

impl Up {
    /// Read a block as command `cid`, which the controller never completes:
    /// its 30 s run out, and the buffer the read would fill is left alone.
    pub fn stall_a_read(&mut self, cid: u16) {
        self.fake.borrow_mut().stall.push(cid);
        let mut buf = [0xa5u8; 512];
        let r = self.io.read_into(self.regs, 0, 1, &mut buf);
        assert!(matches!(r, Err(NvmeError::ControllerTimeout)), "{r:?}");
        assert!(buf.iter().all(|&b| b == 0xa5), "a buffer was handed back");
        assert_eq!(self.io.stuck(), Some(cid));
    }

    pub fn recover(&mut self, hmb: Option<[u32; 5]>) -> Recovered {
        let (device, admin, io) = (&mut self.device, &mut self.admin, &mut self.io);
        recover(&mut Stalled { device, admin, io, hmb })
    }
}
