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

use super::request::io_request;
use super::stalled::Serving;
use crate::error::NvmeResult;
use crate::nvm::IoQueue;
use crate::regs::Regs;
use crate::setup::Driver;

impl Driver {
    /// One read, write or flush on this controller's I/O queue (io_request),
    /// with the host memory buffer a reset gives back; `sender` the pid
    /// that sent it, 0 the kernel, whose request a recovery's bound is for.
    pub fn io_command<T: ?Sized, F, R>(&mut self, sender: u32, tx: &mut T, op: F, reply: R)
    where
        F: FnMut(&mut IoQueue, Regs, &mut T) -> NvmeResult<()>,
        R: FnOnce(&mut T, Result<(), i32>),
    {
        let hmb = self.hmb.as_ref().map(|h| h.enable);
        let on =
            Serving { device: &mut self.device, admin: &mut self.admin, io: &mut self.io, hmb };
        io_request(on, sender, tx, op, reply);
    }
}
