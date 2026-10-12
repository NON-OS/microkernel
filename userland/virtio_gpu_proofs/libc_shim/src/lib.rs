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

//! The three things the included driver files take from `nonos_libc`.
//!
//! The register type reaches the part through memory here, so the port I/O
//! pair answers a refusal and the driver's own fallback handles it. `mk_yield`
//! is what the queue wait calls between spins; answering zero lets the wait
//! run to its limit, which is how the no-answer test reaches the timeout.

pub fn mk_yield() -> i64 {
    0
}

pub fn mk_pio_read(_grant_id: u64, _port_offset: u16, _width: u8, _value: &mut u32) -> i64 {
    -1
}

pub fn mk_pio_write(_grant_id: u64, _port_offset: u16, _width: u8, _value: u32) -> i64 {
    -1
}

// The real libc's register accessors and DMA barriers, mounted so the driver
// files under test reach a device through the code they ship with.
#[path = "../../../libc/src/broker/dma_order.rs"]
mod dma_order;
#[path = "../../../libc/src/broker/mmio_access.rs"]
mod mmio_access;
pub use dma_order::{dma_mb, dma_rmb, dma_wmb};
pub use mmio_access::{
    mmio_read16, mmio_read32, mmio_read64, mmio_read8, mmio_write16, mmio_write32, mmio_write64,
    mmio_write8,
};
