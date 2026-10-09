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

//! The SD card in the slot: power, signalling, the identification sequence
//! the Linux MMC core runs through this host's ops, and block reads.

mod app;
mod attach;
mod clock;
mod command;
mod identify;
mod info;
mod power;
mod power_off;
mod present;
mod pull;
mod read;
mod select;
mod timing;
mod up;
mod voltage;

pub use info::Card;
pub use power_off::power_off;
pub use present::present;
pub use read::read_blocks;
pub use up::bring_up;
