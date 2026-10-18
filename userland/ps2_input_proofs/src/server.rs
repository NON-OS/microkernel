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

//! The serve loop's state, its pump and its upkeep between two waits, as
//! shipped. The loop around them (the IPC receive, the interrupt wait and
//! the op handlers) talks to the kernel and stays out: a test calls these
//! where the loop would.

#[path = "../../capsule_driver_ps2_input/src/server/context.rs"]
pub mod context;
#[path = "../../capsule_driver_ps2_input/src/server/irq_seq.rs"]
mod irq_seq;
#[path = "../../capsule_driver_ps2_input/src/server/pump.rs"]
pub mod pump;
#[path = "../../capsule_driver_ps2_input/src/server/reinit.rs"]
pub mod reinit;
#[path = "../../capsule_driver_ps2_input/src/server/reinits.rs"]
mod reinits;
#[path = "../../capsule_driver_ps2_input/src/server/upkeep.rs"]
pub mod upkeep;
