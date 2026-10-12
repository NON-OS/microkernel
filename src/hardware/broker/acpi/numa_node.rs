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

//! Which proximity domain a physical range or a PCI bus lies in. Memory
//! ranges come from SRAT; a range is in a domain only when ranges of that
//! one domain cover every byte of it, joined end to end. Bus ranges come
//! from the ACPI capsule: each PCI root bridge's buses and its _PXM. Pure:
//! `kernel_proofs` mounts it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    pub base: u64,
    pub len: u64,
    pub domain: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BusRange {
    pub segment: u16,
    pub first: u8,
    pub last: u8,
    pub domain: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeError {
    /// Some byte lies in no SRAT memory range.
    Outside,
    /// The range runs across two domains.
    Spans,
}

pub fn node_of(regions: &[Region], base: u64, len: u64) -> Result<u32, NodeError> {
    let end = base.checked_add(len).filter(|_| len != 0).ok_or(NodeError::Outside)?;
    let (mut at, mut domain) = (base, None);
    // each pass moves past one region, or stops
    for _ in 0..=regions.len() {
        if at >= end {
            break;
        }
        let holds = |r: &&Region| r.base <= at && at - r.base < r.len;
        let r = regions.iter().find(holds).ok_or(NodeError::Outside)?;
        if domain.is_some_and(|d| d != r.domain) {
            return Err(NodeError::Spans);
        }
        domain = Some(r.domain);
        at = r.base.saturating_add(r.len);
    }
    domain.filter(|_| at >= end).ok_or(NodeError::Outside)
}

/// A domain as a record's `numa_node` holds it: 0xFF for none, and for
/// any domain past 254, which no byte can name apart from "unknown".
pub fn node_byte(domain: Option<u32>) -> u8 {
    domain.and_then(|d| u8::try_from(d).ok()).unwrap_or(0xFF)
}

/// The domain of the root bridge whose buses hold `bus`.
pub fn bus_domain(ranges: &[BusRange], segment: u16, bus: u8) -> Option<u32> {
    let holds = |r: &&BusRange| r.segment == segment && (r.first..=r.last).contains(&bus);
    ranges.iter().find(holds).map(|r| r.domain)
}
