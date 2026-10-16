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

//! The broker's records, field for field as nonos_libc lays them out.

pub const BAR_KIND_MMIO: u8 = 1;
pub const BAR_KIND_PIO: u8 = 2;

#[derive(Clone, Copy)]
pub struct Bar {
    pub base: u64,
    pub size: u64,
    pub aux: u32,
    pub kind: u8,
    pub flags: u8,
    pub _pad: [u8; 2],
}

impl Bar {
    pub const fn empty() -> Self {
        Self { base: 0, size: 0, aux: 0, kind: 0, flags: 0, _pad: [0; 2] }
    }
}

#[derive(Clone, Copy)]
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

#[derive(Clone, Copy)]
pub struct IrqBindOut {
    pub grant_id: u64,
    pub vector: u64,
}

#[derive(Clone, Copy)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}
