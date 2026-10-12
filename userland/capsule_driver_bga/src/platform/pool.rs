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
//! Shared to a pid once the broker has `mk_surface_share_to` (L18 (c));
//! until then the share is the registry's own, and the wire's owner rule
//! is the only limit on who is answered.

use nonos_gfx_serve::{Platform, Region};
use nonos_gfx_wire::Errno;
use nonos_libc::mk_getpid;

use super::{anon, surface};

pub struct Pool;

impl Platform for Pool {
    fn alloc(&mut self, len: usize) -> Result<Region, Errno> {
        anon::map(len)
    }

    fn register(&mut self, r: Region, w: u32, h: u32, stride: u32) -> Result<u64, Errno> {
        surface::publish(r, w, h, stride)
    }

    fn share(&mut self, _handle: u64, _pid: u32) -> Result<(), Errno> {
        Ok(())
    }

    fn release(&mut self, handle: Option<u64>, r: Region) {
        if let Some(h) = handle {
            surface::withdraw(h);
        }
        anon::unmap(r);
    }

    fn bytes(&self, r: Region) -> &[u8] {
        anon::bytes(r)
    }

    fn pid(&self) -> u32 {
        mk_getpid()
    }
}
