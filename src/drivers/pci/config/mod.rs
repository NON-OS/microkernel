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

pub mod access;
pub mod bridge;
pub mod check;
mod command_edit;
pub mod config_space;
mod counters;
#[cfg(target_arch = "x86_64")]
mod extended;
#[cfg(target_arch = "x86_64")]
mod extended_write;
pub mod power;
mod transport;
mod unchecked;

pub use access::{read16, read32, read8, write16, write32, write8};
pub use bridge::BridgeConfigSpace;
pub use check::check;
pub use config_space::ConfigSpace;
pub use counters::{get_config_stats, reset_config_stats};
#[cfg(target_arch = "x86_64")]
pub(crate) use extended::map_ecam_bus;
pub use transport::ecam::{self, EcamError, EcamWindow};
pub use unchecked::{read32_unchecked, write32_unchecked};
#[cfg(target_arch = "x86_64")]
pub use {extended::read_extended32, extended_write::write_extended32};
