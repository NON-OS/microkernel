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

//! Host proofs for the DEC 21140/21143 (tulip) driver. Every driver file
//! below is the shipping source, included by path; only the longword window
//! under `regs` is replaced, by a model of the part that answers each access
//! the way the 21x4x manuals and QEMU's tulip describe.

#[path = "../../capsule_driver_tulip/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_tulip/src/discover.rs"]
pub mod discover;
pub mod init;
pub mod model;
#[path = "../../capsule_driver_tulip/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_tulip/src/queue/mod.rs"]
pub mod queue;
pub mod regs;
#[path = "../../capsule_driver_tulip/src/report/mod.rs"]
pub mod report;
#[path = "../../capsule_driver_tulip/src/server/handlers/rx_fill.rs"]
pub mod rx_fill;
pub mod setup;
#[path = "../../capsule_driver_tulip/src/server/handlers/tx_stage.rs"]
pub mod tx_stage;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;
