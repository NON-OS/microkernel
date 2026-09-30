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

//! What PxSIG says about a port, before and after its link is brought up.

use crate::constants::regs::{SIG_ATAPI, SIG_PM, SIG_SATA, SIG_SEMB};

/*
 * PxSIG is filled from the device's first D2H Register FIS after a reset. The
 * HBA reset in enable_ahci resets every port, and until that FIS arrives the
 * register reads 0xFFFFFFFF; QEMU delivers it only once FIS receive (FRE) is
 * on, which program() sets later. A signature read right after the reset is
 * therefore no reason to skip a port: only one that names another device kind
 * is.
 */
/// Whether a port with its link up is worth bringing up as a disk, judged from
/// the signature read before FIS receive was enabled.
pub fn may_be_disk(sig: u32) -> bool {
    !matches!(sig, SIG_ATAPI | SIG_SEMB | SIG_PM)
}

/// Whether the device on a port that link_up brought up is an ATA disk. By
/// then the device has sent its D2H FIS, so the signature is the device's own.
pub fn is_ata_disk(sig: u32) -> bool {
    sig == SIG_SATA
}
