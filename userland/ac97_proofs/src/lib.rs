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

//! Host proofs for the ICH AC'97 driver. Every driver file below is the
//! shipping source, included by path, the port windows and the mixer
//! semaphore among them; only the broker under them is the shim's, with
//! the port I/O answered by a model of the controller and its codec.

extern crate alloc;

pub mod audio;
#[path = "../../capsule_driver_ac97/src/codec/mod.rs"]
pub mod codec;
#[path = "../../capsule_driver_ac97/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_ac97/src/discover.rs"]
pub mod discover;
pub mod engine;
pub mod model;
#[path = "../../capsule_driver_ac97/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_ac97/src/regs/mod.rs"]
pub mod regs;
#[path = "../../capsule_driver_ac97/src/say/mod.rs"]
pub mod say;
pub mod server;
pub mod setup;
#[path = "../../capsule_driver_ac97/src/wait.rs"]
pub mod wait;
// The audio server's side of the same wire, which this driver must answer.
#[path = "../../capsule_audio/src/sink/wire.rs"]
pub mod sink_wire;

#[cfg(test)]
mod tests;
