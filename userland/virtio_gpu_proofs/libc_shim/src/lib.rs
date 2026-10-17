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
//! What the included driver files take from `nonos_libc`.
//!
//! The register type reaches the part through memory here, so the port I/O
//! pair answers a refusal and the driver's own fallback handles it. `mk_yield`
//! is what the queue wait calls between spins; answering zero lets the wait
//! run to its limit, which is how the no-answer test reaches the timeout. A
//! test may hang a device step on it instead (`on_yield`), so a model can
//! answer the ring on the driver's own thread, one command at a time (G-73).
//! Replies, DMA and the surface registry are kept per thread for a test to
//! read back.

mod dma;
mod ipc;
mod surface;
mod yield_hook;

pub use dma::{dma_live, dma_reset, mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_HIGH};
pub use ipc::{mk_ipc_reply, take_reply};
pub use surface::{
    mk_surface_register, mk_surface_share, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888,
};
pub use yield_hook::{mk_yield, off_yield, on_yield};

pub fn mk_pio_read(_grant_id: u64, _port_offset: u16, _width: u8, _value: &mut u32) -> i64 {
    -1
}

pub fn mk_pio_write(_grant_id: u64, _port_offset: u16, _width: u8, _value: u32) -> i64 {
    -1
}
