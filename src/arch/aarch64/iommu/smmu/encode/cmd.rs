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

//! Command queue entries: two little-endian doublewords each.

pub type Command = [u64; 2];

const CFGI_STE: u64 = 0x03;
const CFGI_ALL: u64 = 0x04;
const CFGI_CD: u64 = 0x05;
const TLBI_NH_ASID: u64 = 0x11;
const TLBI_NSNH_ALL: u64 = 0x30;
const SYNC: u64 = 0x46;

/// Drop the unit's cached copy of one STE. Leaf: only the STE changed, not
/// a level 1 descriptor above it.
pub const fn cfgi_ste(sid: u32) -> Command {
    [CFGI_STE | (sid as u64) << 32, 1]
}

/// Drop every cached STE and CD; range 31 covers every stream.
pub const fn cfgi_all() -> Command {
    [CFGI_ALL, 31]
}

/// Drop the cached CD for substream 0 of `sid`.
pub const fn cfgi_cd(sid: u32) -> Command {
    [CFGI_CD | (sid as u64) << 32, 1]
}

/// Drop every non-secure stage 1 TLB entry tagged with `asid`.
pub const fn tlbi_nh_asid(asid: u16) -> Command {
    [TLBI_NH_ASID | (asid as u64) << 48, 0]
}

/// Drop every non-secure, non-hypervisor TLB entry.
pub const fn tlbi_nsnh_all() -> Command {
    [TLBI_NSNH_ALL, 0]
}

/// Complete once every earlier command has; signals nothing, so completion
/// is seen as CONS passing it.
pub const fn sync() -> Command {
    [SYNC, 0]
}

/// CMDQ_CONS.ERR: the reason the unit stopped at a command, zero if none.
pub const fn cons_error(cons: u32) -> u8 {
    ((cons >> 24) & 0x7f) as u8
}
