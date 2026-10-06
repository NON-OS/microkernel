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

pub const VER: usize = 0x000;
pub const CAP: usize = 0x008;
pub const ECAP: usize = 0x010;
pub const GCMD: usize = 0x018;
pub const GSTS: usize = 0x01C;
pub const RTADDR: usize = 0x020;
pub const FSTS: usize = 0x034;
pub const FECTL: usize = 0x038;
/// Protected Memory Enable. Firmware uses the protected memory regions for
/// pre-boot DMA protection and may leave them on at handoff.
pub const PMEN: usize = 0x064;
pub const PMEN_EPM: u32 = 1 << 31;
pub const PMEN_PRS: u32 = 1 << 0;

/// GCMD is write-only, not read-modify-write: every write sets one command and
/// must carry the state of the others, which is what GSTS is read for.
pub const GCMD_TE: u32 = 1 << 31;
pub const GCMD_SRTP: u32 = 1 << 30;
pub const GCMD_WBF: u32 = 1 << 27;

/// GCMD bits that start a one-shot operation rather than set a control:
/// SRTP (30), SFL (29), WBF (27) and SIRTP (24). Their GSTS counterparts report
/// the last such operation, so a GCMD built from GSTS must clear them or every
/// later command silently repeats them (VT-d spec 10.4.4: software reads GSTS
/// and masks it with 0x96FF_FFFF before writing GCMD).
pub const GCMD_ONE_SHOT: u32 = (1 << 30) | (1 << 29) | (1 << 27) | (1 << 24);

/// The GCMD value that issues `command` while keeping every persistent control
/// as `status` (a fresh GSTS read) reports it, and repeating no one-shot.
pub const fn gcmd_with(status: u32, command: u32) -> u32 {
    (status & !GCMD_ONE_SHOT) | command
}

pub const GSTS_TES: u32 = 1 << 31;
pub const GSTS_RTPS: u32 = 1 << 30;
pub const GSTS_WBFS: u32 = 1 << 27;

/// The GCMD value that turns translation off while keeping every other
/// persistent control as `status` reports it.
pub const fn gcmd_without_te(status: u32) -> u32 {
    status & !GCMD_ONE_SHOT & !GCMD_TE
}
