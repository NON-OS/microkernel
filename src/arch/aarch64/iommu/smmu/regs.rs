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

//! Register offsets and the bits the driver writes. Page 0 is at the base;
//! page 1, which holds the event queue's PROD and CONS, is 64 KiB above.

pub(super) const IDR0: u64 = 0x00;
pub(super) const IDR1: u64 = 0x04;
pub(super) const IDR5: u64 = 0x14;
pub(super) const CR0: u64 = 0x20;
pub(super) const CR0ACK: u64 = 0x24;
pub(super) const CR1: u64 = 0x28;
pub(super) const CR2: u64 = 0x2c;
pub(super) const GBPA: u64 = 0x44;
pub(super) const GERROR: u64 = 0x60;
pub(super) const GERRORN: u64 = 0x64;
pub(super) const STRTAB_BASE: u64 = 0x80;
pub(super) const STRTAB_BASE_CFG: u64 = 0x88;
pub(super) const CMDQ_BASE: u64 = 0x90;
pub(super) const CMDQ_PROD: u64 = 0x98;
pub(super) const CMDQ_CONS: u64 = 0x9c;
pub(super) const EVENTQ_BASE: u64 = 0xa0;
pub(super) const EVENTQ_PROD: u64 = 0x1_00a8;
pub(super) const EVENTQ_CONS: u64 = 0x1_00ac;

/// Both 64 KiB register pages.
pub(super) const WINDOW: usize = 0x2_0000;

pub(super) const CR0_SMMUEN: u32 = 1 << 0;
pub(super) const CR0_EVENTQEN: u32 = 1 << 2;
pub(super) const CR0_CMDQEN: u32 = 1 << 3;

/// Queues and tables: inner shareable, inner and outer write-back, the
/// same attributes the kernel's own mapping of them has.
pub(super) const CR1_VALUE: u32 = 0b11 << 10 | 0b01 << 8 | 0b01 << 6 | 0b11 << 4 | 0b01 << 2 | 0b01;
/// Record invalid stream IDs; private TLB maintenance only.
pub(super) const CR2_VALUE: u32 = 1 << 1 | 1 << 2;

pub(super) const GBPA_UPDATE: u32 = 1 << 31;
pub(super) const GBPA_ABORT: u32 = 1 << 20;

/// The read-allocate hint queue and table base registers take.
pub(super) const BASE_RA: u64 = 1 << 62;
/// STRTAB_BASE_CFG.FMT for a two-level table.
pub(super) const STRTAB_FMT_2LVL: u32 = 1 << 16;
