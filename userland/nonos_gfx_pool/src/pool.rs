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
//! Each primary is shared with the compositor when it is registered
//! (surface.rs, K20). A share right dies with the pid that held it, so a
//! compositor started again gets its own from `share`: the pool keeps every
//! handle's sid, the one name the kernel shares by, and never hands it out.

extern crate alloc;

use alloc::vec::Vec;

use nonos_gfx_serve::{Platform, Region};
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::Errno;
use nonos_libc::mk_getpid;

use super::{anon, receiver, surface};

#[derive(Default)]
pub struct Pool {
    /// (handle, sid) for every primary registered and not yet released.
    sids: Vec<(u64, u64)>,
}

impl Platform for Pool {
    fn alloc(&mut self, len: usize) -> Result<Region, Errno> {
        anon::map(len)
    }

    fn register(&mut self, r: Region, w: u32, h: u32, stride: u32) -> Result<u64, Errno> {
        let (handle, sid) = surface::publish(r, w, h, stride)?;
        self.sids.push((handle, sid));
        Ok(handle)
    }

    fn share(&mut self, handle: u64, pid: u32) -> Result<(), Errno> {
        let &(_, sid) = self.sids.iter().find(|(h, _)| *h == handle).ok_or(EINVAL)?;
        receiver::reshare(sid, pid)
    }

    fn release(&mut self, handle: Option<u64>, r: Region) {
        if let Some(h) = handle {
            self.sids.retain(|(held, _)| *held != h);
            surface::withdraw(h);
        }
        anon::unmap(r);
    }

    fn bytes(&self, r: Region) -> &[u8] {
        anon::bytes(r)
    }

    fn scrub(&mut self, r: Region) {
        anon::scrub(r)
    }

    fn pid(&self) -> u32 {
        mk_getpid()
    }
}
