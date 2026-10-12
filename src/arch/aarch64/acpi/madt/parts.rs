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

//! The GIC Distributor (0x0C, 24 bytes), Redistributor (0x0E, 16 bytes)
//! and ITS (0x0F, 20 bytes) structures (ACPI 6.5, 5.2.12.15, .17, .18).

use super::super::le::{u32_at, u64_at, u8_at};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gicd {
    pub id: u32,
    pub base: u64,
    /// 1 to 4 for GICv1 to GICv4; 0 means look at the hardware.
    pub version: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gicr {
    pub base: u64,
    pub length: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Its {
    pub id: u32,
    pub base: u64,
}

pub const GICD_LEN: usize = 24;
pub const GICR_LEN: usize = 16;
pub const ITS_LEN: usize = 20;

pub fn gicd(s: &[u8]) -> Option<Gicd> {
    Some(Gicd { id: u32_at(s, 4)?, base: u64_at(s, 8)?, version: u8_at(s, 20)? })
}

pub fn gicr(s: &[u8]) -> Option<Gicr> {
    Some(Gicr { base: u64_at(s, 4)?, length: u32_at(s, 12)? })
}

pub fn its(s: &[u8]) -> Option<Its> {
    Some(Its { id: u32_at(s, 4)?, base: u64_at(s, 8)? })
}
