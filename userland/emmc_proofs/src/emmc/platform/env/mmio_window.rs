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

//! The slot register window on the host: the model host plugged in on this
//! thread (`model::with`) answers every access, whatever the base.

use crate::emmc::env::Mmio;
use crate::model::with;

/// The slot's registers, with the capsule's constructor.
#[derive(Clone, Copy)]
pub struct MmioWindow {
    _base: u64,
}

impl MmioWindow {
    /// # Safety
    /// None on the host: nothing at `base` is dereferenced.
    pub const unsafe fn new(base: u64) -> Self {
        Self { _base: base }
    }
}

impl Mmio for MmioWindow {
    fn r8(&self, off: u32) -> u8 {
        with(|h| h.read(off, 1)) as u8
    }
    fn r16(&self, off: u32) -> u16 {
        with(|h| h.read(off, 2)) as u16
    }
    fn r32(&self, off: u32) -> u32 {
        with(|h| h.read(off, 4))
    }
    fn w8(&self, off: u32, v: u8) {
        with(|h| h.write(off, 1, v.into()))
    }
    fn w16(&self, off: u32, v: u16) {
        with(|h| h.write(off, 2, v.into()))
    }
    fn w32(&self, off: u32, v: u32) {
        with(|h| h.write(off, 4, v))
    }
}
