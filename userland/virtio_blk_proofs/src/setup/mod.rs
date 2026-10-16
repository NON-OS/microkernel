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

//! The driver's bring-up as it ships, every file mounted on its own so the
//! modern capacity read is reachable from the tests (`capacity_tests`); the
//! shipping setup/mod.rs keeps it private to bring-up.

#[path = "../../../capsule_driver_virtio_blk/src/setup/claim.rs"]
mod claim;
#[path = "../../../capsule_driver_virtio_blk/src/setup/dma.rs"]
mod dma;
#[path = "../../../capsule_driver_virtio_blk/src/setup/driver.rs"]
mod driver;
#[path = "../../../capsule_driver_virtio_blk/src/setup/irq.rs"]
mod irq;
#[path = "../../../capsule_driver_virtio_blk/src/setup/legacy.rs"]
mod legacy;
pub mod modern;
#[path = "../../../capsule_driver_virtio_blk/src/setup/pci.rs"]
mod pci;
#[path = "../../../capsule_driver_virtio_blk/src/setup/quiesce.rs"]
mod quiesce;
#[path = "../../../capsule_driver_virtio_blk/src/setup/registers.rs"]
mod registers;
#[path = "../../../capsule_driver_virtio_blk/src/setup/sequence.rs"]
mod sequence;
#[path = "../../../capsule_driver_virtio_blk/src/setup/step.rs"]
mod step;

pub use driver::Driver;
pub use sequence::run;
