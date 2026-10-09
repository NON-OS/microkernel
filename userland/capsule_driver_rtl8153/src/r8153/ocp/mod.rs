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

//! The register layer: Linux get_registers and set_registers, the dword,
//! word and byte accessors built on them, the PHY behind the OCP window,
//! and waiting for the chip on the clock.

mod dev;
mod phy;
mod read;
mod request;
mod update;
mod wait;
mod write;

pub use dev::Dev;
pub use phy::{mdio_read, mdio_write, phy_read, phy_write, sram_write};
pub use read::{get, read_byte, read_dword, read_word};
pub use request::{BYTE_EN_DWORD, BYTE_EN_WORD};
pub use request::{MCU_TYPE_PLA as PLA, MCU_TYPE_USB as USB};
pub use update::{update_byte, update_dword, update_word};
pub use wait::wait_until;
pub use write::{set, write_byte, write_dword, write_word};
