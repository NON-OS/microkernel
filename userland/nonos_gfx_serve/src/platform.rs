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

//! The kernel half a driver writes: memory for a primary, the surface
//! registry, and sharing with the compositor. The capsule implements it
//! over its syscalls; the proofs over a model of the registry.

use nonos_gfx_wire::Errno;

/// Memory the platform handed out: its base for the driver to read, and
/// its length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    pub base: usize,
    pub len: usize,
}

pub trait Platform {
    /// Zeroed, page-aligned memory of at least `len` bytes; ENOMEM when
    /// the display pool cannot hold it.
    fn alloc(&mut self, len: usize) -> Result<Region, Errno>;
    /// Register `r` as a `w` x `h` ARGB8888 surface of `stride`; its
    /// handle.
    fn register(&mut self, r: Region, w: u32, h: u32, stride: u32) -> Result<u64, Errno>;
    /// Let `pid` attach the surface.
    fn share(&mut self, handle: u64, pid: u32) -> Result<(), Errno>;
    /// Unregister the surface (when it has a handle) and free `r`.
    fn release(&mut self, handle: Option<u64>, r: Region);
    /// The bytes of a region this platform handed out.
    fn bytes(&self, r: Region) -> &[u8];
    /// This driver's pid, the high half of its serials.
    fn pid(&self) -> u32;
}
