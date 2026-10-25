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

//! The region rule, pure: `kernel_proofs` mounts this directory as it is.

mod allowed;
mod bridge;
mod facts;
mod legacy;
mod memory;
mod ports;
mod power;
mod reason;
mod refusal;
mod sleep_enable;
mod table;

pub use allowed::AML_PORTS;
pub use bridge::{bridge_write, tell, Tell, ROUTING};
pub use facts::{Facts, MemKind};
pub use legacy::LEGACY_PORTS;
pub use memory::memory;
pub use ports::{io, pci_config};
pub use power::{power_reg, touches as power_touches, PowerReg, SPACE_IO, SPACE_MEMORY};
pub use refusal::{Hardware, Legacy, Refusal};
pub use sleep_enable::{
    sets_sleep_enable, sleep_enable, SleepEnable, PM1_SLP_EN, SLEEP_CONTROL_SLP_EN,
};
pub use table::{table_read, LEGACY_BIOS};
