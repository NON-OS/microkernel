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

//! The server's side of a request, on the fake: the driver's own io_request
//! over queue 1, and each reply noted as the server sends it.

use std::{cell::RefCell, rc::Rc};

use super::inbox::Reply;
use super::{io_request, Serving};
use crate::admin::AdminQueue;
use crate::fake::{register, Fake, Up};
use crate::nvm::{sq_tail_doorbell, IoQueue};
use crate::setup::Device;

pub(super) struct Server {
    pub fake: Rc<RefCell<Fake>>,
    pub device: Device,
    pub admin: AdminQueue,
    pub io: Option<IoQueue>,
    /// Each request writes the block rather than reading it.
    pub writes: bool,
}

impl Server {
    pub fn new(up: Up) -> Self {
        let Up { fake, device, admin, io, .. } = up;
        Self { fake, device, admin, io: Some(io), writes: false }
    }

    /// Read one block for the kernel as the server does; the clock moves
    /// 100 ms a read.
    pub fn read(&mut self, hmb: Option<[u32; 5]>) -> Reply {
        self.read_as(hmb, 0)
    }

    /// `read`, or a write (`writes`), sent by the pid `sender` (0 the kernel).
    pub fn read_as(&mut self, hmb: Option<[u32; 5]>, sender: u32) -> Reply {
        let (fake, start, regs) = (self.fake.clone(), self.fake.borrow().ticks, self.device.regs);
        let (mut tx, mut said, writes) = ([0u8; 512], None, self.writes);
        let op = |io: &mut IoQueue, regs, tx: &mut [u8]| {
            if writes {
                io.write_from(regs, 0, 1, tx)
            } else {
                io.read_into(regs, 0, 1, tx)
            }
        };
        let reply = |_: &mut [u8], status| {
            let (f, sent) = (fake.borrow(), register(regs, sq_tail_doorbell(1, 0)));
            let (ms, at, admin) = ((f.ticks - start) * 100, f.ticks * 100, f.admin_seen.len());
            said = Some(Reply { status, ms, at, admin, sent });
        };
        let on =
            Serving { device: &mut self.device, admin: &mut self.admin, io: &mut self.io, hmb };
        io_request(on, sender, &mut tx[..], op, reply);
        said.expect("every request is answered")
    }
}
