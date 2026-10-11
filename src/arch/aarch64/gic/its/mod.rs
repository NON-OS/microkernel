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

//! The GICv3 ITS: message signalled interrupts as LPIs, translated from the
//! id of the device that wrote and the event it wrote.

mod bring_up;
mod config;
mod device;
pub mod encode;
mod lpi_tables;
mod lpis;
mod memory;
mod queue;
mod queue_run;
mod regs;
mod slots;
mod state;
mod tables;
mod vectors;

pub use bring_up::bring_up;
pub use config::enable;
pub use vectors::{bind, unbind, MAX_VECTORS};
