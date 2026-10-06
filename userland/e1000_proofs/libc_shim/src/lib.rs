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

//! The things the included driver files take from `nonos_libc`.
//!
//! `crypto_random` is the one that matters. The station address is drawn
//! from it, and the driver's promise is that when there is no entropy the
//! card is never brought on the air under the address in its EEPROM. The
//! tests switch it off to hold the driver to that.

mod deadline;
mod debug;
mod entropy;
mod grants;

pub use deadline::Deadline;
pub use debug::{logged, mk_debug};
pub use entropy::{crypto_random, entropy, Entropy};
pub use grants::{given_back, mk_device_release, mk_dma_unmap, mk_irq_unbind, mk_mmio_unmap};
