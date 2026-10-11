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

//! Every hardware value here is copied from Linux v6.6
//! drivers/net/ethernet/broadcom/tg3.h or tg3.c under the name Linux gives
//! it, or from include/uapi/linux/mii.h and pci_regs.h for the standard
//! registers. A value no source could be found for is not written at all.

pub mod blocks;
pub mod bufmgr;
pub mod chip;
pub mod cpmu;
pub mod desc;
mod frame;
pub mod grc;
pub mod hostcc;
pub mod layout;
pub mod literals;
pub mod mac;
pub mod mailbox;
pub mod mi;
pub mod pci;
pub mod pcicfg;
pub mod phy;
pub mod ring;
pub mod sram;
pub mod timeouts;

pub use frame::{FCS_LEN, MAC_LEN, MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
