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

//! The 128-bit commands the kernel puts in a command buffer (AMD IOMMU spec
//! 48882, section 2.4), as four dwords, laid out as Linux builds them in
//! amd/iommu.c (build_completion_wait, build_inv_dte, build_inv_iommu_pages,
//! build_inv_all). The opcode sits in bits 31:28 of the second dword.

pub type Command = [u32; 4];

const COMPLETION_WAIT: u32 = 0x1;
const INVALIDATE_DEVTAB_ENTRY: u32 = 0x2;
const INVALIDATE_IOMMU_PAGES: u32 = 0x3;
const INVALIDATE_IOMMU_ALL: u32 = 0x8;

const fn opcode(op: u32) -> u32 {
    op << 28
}

/// Once every earlier command is done, store `data` at the 8-byte aligned
/// `store_phys` (S, bit 0). The kernel polls that quadword on the clock.
pub const fn completion_wait(store_phys: u64, data: u64) -> Command {
    let address = store_phys & 0x000F_FFFF_FFFF_FFF8;
    [
        (address as u32) | 1,
        ((address >> 32) as u32 & 0xF_FFFF) | opcode(COMPLETION_WAIT),
        data as u32,
        (data >> 32) as u32,
    ]
}

/// Drop the unit's cached copy of the device table entry for `device_id`.
pub const fn invalidate_devtab_entry(device_id: u16) -> Command {
    [device_id as u32, opcode(INVALIDATE_DEVTAB_ENTRY), 0, 0]
}

/// Every cached translation and page directory entry of `domain`: the
/// all-pages address 0x7FFF_FFFF_FFFF_F000 with S (bit 0) and PDE (bit 1).
pub const fn invalidate_domain_pages(domain: u16) -> Command {
    [0, domain as u32 | opcode(INVALIDATE_IOMMU_PAGES), 0xFFFF_F000 | 0b11, 0x7FFF_FFFF]
}

/// Everything the unit caches, on units that report EFR.IASup.
pub const fn invalidate_all() -> Command {
    [0, opcode(INVALIDATE_IOMMU_ALL), 0, 0]
}

/// The opcode a command carries, for the proofs and for the refusal line.
pub const fn command_opcode(command: Command) -> u32 {
    command[1] >> 28
}
