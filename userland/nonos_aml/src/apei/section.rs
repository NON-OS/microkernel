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

//! The section types NONOS reads (UEFI 2.10, appendix N.2): processor
//! generic, platform memory and PCI Express errors, each to the fields a
//! log line needs, and only those its validation bits say are valid.

use super::guids::{MEMORY, PCIE, PROCESSOR};
use super::le::{u16_at, u32_at, u64_at};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Record {
    /// Error type bits (1 cache, 2 TLB, 4 bus, 8 micro-architectural),
    /// cache level, and the processor's id when valid.
    Processor { error_type: u8, level: u8, processor: Option<u64> },
    /// The physical address and its DIMM coordinates when valid.
    Memory { address: Option<u64>, node: u16, card: u16, module: u16, bank: u16, kind: u8 },
    /// The function, its ids, and AER's uncorrectable status when valid.
    Pcie {
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        vendor: u16,
        id: u16,
        uncorrectable: Option<u32>,
    },
    /// A section of another type, or too short for its own.
    Other,
}

fn valid(d: &[u8], bit: u32) -> bool {
    u64_at(d, 0) & (1 << bit) != 0
}

pub fn record(section: &[u8; 16], d: &[u8]) -> Record {
    match *section {
        PROCESSOR if d.len() >= 160 => Record::Processor {
            error_type: if valid(d, 2) { d[10] } else { 0 },
            level: if valid(d, 5) { d[13] } else { 0 },
            processor: valid(d, 8).then(|| u64_at(d, 152)),
        },
        MEMORY if d.len() >= 73 => Record::Memory {
            address: valid(d, 1).then(|| u64_at(d, 16)),
            node: u16_at(d, 32),
            card: u16_at(d, 34),
            module: u16_at(d, 36),
            bank: u16_at(d, 38),
            kind: if valid(d, 14) { d[72] } else { 0 },
        },
        PCIE if d.len() >= 120 => Record::Pcie {
            segment: u16_at(d, 33),
            bus: d[35],
            device: d[32],
            function: d[31],
            vendor: u16_at(d, 24),
            id: u16_at(d, 26),
            uncorrectable: valid(d, 7).then(|| u32_at(d, 116)),
        },
        _ => Record::Other,
    }
}
