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
//! generic, platform memory and PCI Express errors, each to its error type
//! and nothing that locates the error: no physical address, no DIMM
//! position, no device ids. The layouts are recalled, unverified; the
//! proofs share this reading.

use super::guids::{MEMORY, PCIE, PROCESSOR};
use super::le::{u32_at, u64_at};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Record {
    /// Error type bits (1 cache, 2 TLB, 4 bus, 8 micro-architectural) and
    /// the cache level, each 0 when not valid.
    Processor { error_type: u8, level: u8 },
    /// The memory error type (N.2.5), when valid.
    Memory { kind: Option<u8> },
    /// AER's uncorrectable error status bits, when valid.
    Pcie { uncorrectable: Option<u32> },
    /// A section of another type, or too short for its own.
    Other,
}

fn valid(d: &[u8], bit: u32) -> bool {
    u64_at(d, 0) & (1 << bit) != 0
}

pub fn record(section: &[u8; 16], d: &[u8]) -> Record {
    match *section {
        PROCESSOR if d.len() >= 14 => Record::Processor {
            error_type: if valid(d, 2) { d[10] } else { 0 },
            level: if valid(d, 5) { d[13] } else { 0 },
        },
        MEMORY if d.len() >= 73 => Record::Memory { kind: valid(d, 14).then(|| d[72]) },
        PCIE if d.len() >= 120 => {
            Record::Pcie { uncorrectable: valid(d, 7).then(|| u32_at(d, 116)) }
        }
        _ => Record::Other,
    }
}
