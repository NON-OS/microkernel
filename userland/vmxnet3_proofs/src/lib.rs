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

//! Host proofs for the vmxnet3 driver. Every driver file below is the
//! shipping source, included by path; only the 32-bit window under `regs`
//! is replaced, by a model of the device that answers each access as
//! vmxnet3_defs.h and QEMU's vmxnet3 describe.

#[path = "../../capsule_driver_vmxnet3/src/constants/mod.rs"]
pub mod constants;
pub mod init;
pub mod model;
#[path = "../../capsule_driver_vmxnet3/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_vmxnet3/src/queue/mod.rs"]
pub mod queue;
pub mod regs;
#[path = "../../capsule_driver_vmxnet3/src/report/mod.rs"]
pub mod report;
#[path = "../../capsule_driver_vmxnet3/src/server/handlers/rx_fill.rs"]
pub mod rx_fill;
pub mod setup;
#[path = "../../capsule_driver_vmxnet3/src/shared/mod.rs"]
pub mod shared;
#[path = "../../capsule_driver_vmxnet3/src/server/handlers/tx_stage.rs"]
pub mod tx_stage;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;
