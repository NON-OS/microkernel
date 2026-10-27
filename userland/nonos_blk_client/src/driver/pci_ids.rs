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

//! The Intel storage functions the disk list names by id, whatever class
//! they report.

/// Intel Volume Management Device functions. Some report the RAID
/// subclass, some a system peripheral class; the id names them either way.
/// Every VMD the kernel brings up (src/drivers/pci/vmd/domain/ids.rs, a
/// proof holds the two together), and 28C1, which it does not, so the disk
/// list tells a person with one to turn VMD off rather than show nothing.
pub const INTEL_VMD: [u16; 14] = [
    0x201D, 0x28C0, 0x28C1, 0x467F, 0x4C3D, 0x7D0B, 0x9A0B, 0xA77F, 0xAD0B, 0xB06F, 0xB07F, 0xB60B,
    0xD70B, 0xD73B,
];

/// Intel eMMC hosts, as the kernel's inventory lists them
/// (src/hardware/inventory/emmc.rs); a proof holds the two lists together.
pub const INTEL_EMMC: [u16; 13] = [
    0x0F14, 0x0F50, 0x2294, 0x0ACC, 0x1AA8, 0x5ACC, 0x31CC, 0x9D2B, 0x9DC4, 0x34C4, 0x18DB, 0x4B47,
    0x4DC4,
];

/// Intel SATA controllers in RAID (RST) mode, named by id as well as by
/// the RAID subclass they report.
pub const INTEL_SATA_RAID: [u16; 2] = [0x2822, 0x282A];
