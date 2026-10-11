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

//! Which PCI functions are eMMC hosts. Pure, so the choice is proven on the
//! host.
//!
//! An SD Host Controller is PCI class 0x08 (base system peripheral),
//! subclass 0x05, prog-if 0x00 (no DMA), 0x01 (DMA) or 0x02 (vendor
//! specific). The broker files class 0x08 under its OTHER id, so discovery
//! lists every device and matches here.
//!
//! Intel's SoCs carry up to three such functions: the eMMC host, the SD card
//! reader and the SDIO host (for the WLAN). Only the eMMC host is the
//! internal disk. The ids below are the ones Linux's sdhci-pci-core.c binds
//! to its eMMC, SD and SDIO fixups; only ids known from that table are
//! listed. Any other SDHCI function is a candidate only when its
//! capabilities name an embedded slot, which `platform` checks after the
//! window is mapped.

mod classify;
mod ids;
mod slot_info;

pub use classify::{classify, Kind};
pub use ids::{INTEL_EMMC, INTEL_NOT_EMMC};
pub use slot_info::{first_bar, PCI_SLOT_INFO};
