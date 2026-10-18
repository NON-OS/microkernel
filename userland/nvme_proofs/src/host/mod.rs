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

//! The driver's own clock, DMA region, register access, console and broker
//! handles, over the libc shim: its queues run on host memory with the test
//! as the controller.

#[path = "../../../capsule_driver_nvme/src/clock/mod.rs"]
pub mod clock;
#[path = "../../../capsule_driver_nvme/src/dma/mod.rs"]
pub mod dma;
#[path = "../../../capsule_driver_nvme/src/handles/mod.rs"]
pub mod handles;
#[path = "../../../capsule_driver_nvme/src/log/mod.rs"]
pub mod log;
#[path = "../../../capsule_driver_nvme/src/regs/mod.rs"]
pub mod regs;
