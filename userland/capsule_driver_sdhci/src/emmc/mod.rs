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

//! eMMC on an SD Host Controller (SDHCI 3.0/4.x host, JEDEC eMMC 5.1 card),
//! served as a block device of 512-byte sectors.
//!
//! The tree is self-contained: every path inside it is relative (`super::`),
//! and only `platform` reaches outside, to `nonos_libc` and `alloc`. So the
//! directory drops into any capsule crate as `src/emmc/` unchanged, and the
//! host proof crate includes every other file with `#[path]`.
//!
//! - `env`: the three things the engine needs from its surroundings, an MMIO
//!   window (`Mmio`), a millisecond clock (`Clock`) and a console line
//!   (`Log`), plus `DmaBuf`, one DMA region as the CPU and the device see it.
//! - `sdhci`: the host controller: registers, capabilities, reset, power,
//!   clock, the command and transfer mode words, ADMA2 descriptors and the
//!   polled command engine.
//! - `mmc`: the card: OCR, CID, CSD, EXT_CSD and R1 decoding, bring-up from
//!   power on to transfer state, the speed and bus width choice, and block
//!   read, write and cache flush.
//! - `disk`: `EmmcDisk`, the surface a server calls.
//! - `info`: the reply payloads (DEVICE_INFO, CONTROLLER_INFO, PORT_LIST).
//! - `pci`: which PCI functions are eMMC hosts.
//! - `platform`: discovery, claim, MMIO and DMA through the broker.

pub mod disk;
pub mod env;
pub mod error;
pub mod info;
pub mod mmc;
pub mod pci;
pub mod platform;
pub mod sdhci;
pub mod text;

pub use disk::SECTOR_SIZE;
pub use error::{reason, wire_status};
pub use platform::{discover, open, Found, Opened};
