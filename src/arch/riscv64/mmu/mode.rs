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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmuMode {
    Bare,
    Sv39,
    Sv48,
    Sv57,
    Unknown,
}

/* The mode the kernel builds every page table for. */
pub const KERNEL_MMU_MODE: MmuMode = MmuMode::Sv39;
const _: () = assert!(KERNEL_MMU_MODE.satp_mode().is_some());

impl MmuMode {
    /* None for Unknown: encoding it as 0 would turn translation off. */
    pub const fn satp_mode(&self) -> Option<usize> {
        match self {
            MmuMode::Bare => Some(0),
            MmuMode::Sv39 => Some(8),
            MmuMode::Sv48 => Some(9),
            MmuMode::Sv57 => Some(10),
            MmuMode::Unknown => None,
        }
    }

    pub fn va_bits(&self) -> usize {
        match self {
            MmuMode::Bare => 64,
            MmuMode::Sv39 => 39,
            MmuMode::Sv48 => 48,
            MmuMode::Sv57 => 57,
            MmuMode::Unknown => 0,
        }
    }

    pub fn levels(&self) -> usize {
        match self {
            MmuMode::Bare => 0,
            MmuMode::Sv39 => 3,
            MmuMode::Sv48 => 4,
            MmuMode::Sv57 => 5,
            MmuMode::Unknown => 0,
        }
    }
}
