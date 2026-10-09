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

//! Register addresses and bits of the Realtek PCIe card readers, as
//! include/linux/rtsx_pci.h names them. The host registers sit in the
//! memory BAR; every other register is internal and reached through HAIMR
//! or the command buffer. Pure.

pub mod card;
pub mod clk;
pub mod host;
pub mod ocp;
pub mod phy;
pub mod pm;
pub mod sd;
