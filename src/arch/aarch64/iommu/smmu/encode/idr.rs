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

//! What an SMMUv3's ID registers say, and whether this driver can use it.

pub use super::oas::oas_bits;

/// The features the driver relies on, read from IDR0, IDR1 and IDR5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Features {
    /// Stream ID bits the unit decodes.
    pub sid_bits: u8,
    /// log2 of the largest command and event queues it takes.
    pub cmdq_log2: u8,
    pub eventq_log2: u8,
    /// Output address size, as the 3-bit code a CD's IPS field takes.
    pub oas_code: u8,
    pub asid16: bool,
}

/// Why a unit is not one this driver programs. Each is a missing feature,
/// never a fault, so the boot goes on with DMA stated as unrestricted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unusable {
    NoStage1,
    NoAarch64Tables,
    NoTwoLevelStreamTable,
    NotLittleEndian,
    NotCoherent,
    No4kGranule,
}

pub fn decode(idr0: u32, idr1: u32, idr5: u32) -> Result<Features, Unusable> {
    if idr0 & (1 << 1) == 0 {
        return Err(Unusable::NoStage1);
    }
    if (idr0 >> 2) & 0b11 < 0b10 {
        return Err(Unusable::NoAarch64Tables);
    }
    if (idr0 >> 27) & 0b11 != 0b01 {
        return Err(Unusable::NoTwoLevelStreamTable);
    }
    // Mixed (0b00) or little-endian only (0b10); big-endian only cannot
    // read the tables this kernel writes.
    if (idr0 >> 21) & 0b11 == 0b11 {
        return Err(Unusable::NotLittleEndian);
    }
    if idr0 & (1 << 4) == 0 {
        return Err(Unusable::NotCoherent);
    }
    if idr5 & (1 << 4) == 0 {
        return Err(Unusable::No4kGranule);
    }
    Ok(Features {
        sid_bits: (idr1 & 0x3f) as u8,
        cmdq_log2: ((idr1 >> 21) & 0x1f) as u8,
        eventq_log2: ((idr1 >> 16) & 0x1f) as u8,
        oas_code: (idr5 & 0b111).min(0b110) as u8,
        asid16: idr0 & (1 << 12) != 0,
    })
}
