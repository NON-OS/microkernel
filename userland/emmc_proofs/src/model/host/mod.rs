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

//! A model SD Host Controller, written from the SD Host Controller
//! Simplified Specification 3.00 and not from the driver: register file,
//! resets, power, clock, the command engine with its response checks, ADMA2
//! walking real host memory, interrupt status, and the rules a driver must
//! keep (clock limits per card state, the response flags each command
//! needs, transfer mode and block registers matching the command, no Nop
//! descriptor with End). A broken rule is recorded in `violations`.

mod answer;
mod bus;
mod cfg;
mod check;
mod clock;
mod command;
mod data;
mod data_check;
mod dev;
mod dma;
mod lie;
mod mmio;
mod power;
mod present;
mod read;
mod regs;
mod sim;
mod slot_events;
mod types;
mod walk;
mod walk_desc;
mod write;

pub use bus::SimBus;
pub use cfg::{HostCfg, GLK_CAPS};
pub use sim::Sim;
pub use types::CmdRec;
