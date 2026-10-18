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
//! The model a driver run is held against, and the rig that wires the real
//! driver to it: host memory standing in for DMA regions, a clock that
//! moves one millisecond per read, and a log the tests read back.

pub mod armed;
mod armed_bus;
pub mod card;
mod card_dev;
mod card_flags;
pub mod dev;
pub mod host;
mod rig;
mod rig_driver;
mod rig_read;
pub mod sd;
mod sd_rig;
mod test_env;

pub use armed::Armed;
pub use armed_bus::with;
pub use card::{CardCfg, St};
pub use host::{HostCfg, SimBus, GLK_CAPS};
pub use rig::{Rig, DATA_BUS};
pub use sd_rig::sd_slot;
pub use test_env::{TestClock, TestDisk, TestLog};
