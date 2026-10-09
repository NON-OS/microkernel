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

//! Host-runnable proofs for the e1000e driver. The driver's own files are
//! included through `#[path]` and run unchanged against host memory and a
//! modelled register window (`nonos_devmodel`); `nonos_libc` is the shim in
//! `libc_shim`.

#[path = "../../capsule_driver_e1000e/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_e1000e/src/log/mod.rs"]
pub mod log;
#[path = "../../capsule_driver_e1000e/src/phy/mod.rs"]
pub mod phy;
#[path = "../../capsule_driver_e1000e/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_e1000e/src/queue/mod.rs"]
pub mod queue;
#[path = "../../capsule_driver_e1000e/src/regs.rs"]
pub mod regs;
#[path = "../../capsule_driver_e1000e/src/swflag/mod.rs"]
pub mod swflag;
#[path = "../../capsule_driver_e1000e/src/wait.rs"]
pub mod wait;

pub mod init;
pub mod server;
pub mod setup;

#[cfg(test)]
mod model;
#[cfg(test)]
mod tests;
