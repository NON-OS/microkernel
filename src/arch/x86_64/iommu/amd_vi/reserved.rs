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

//! The domains AMD-Vi bring-up gives unclaimed devices, as K2 does on
//! VT-d. None is identity mapped: a device firmware reserved no memory for
//! stays blocked until a capsule claims it; one with IVMD ranges gets a
//! domain holding those and nothing else, shared by every device whose
//! ranges are the same. Run once the units are in service, since mapping
//! flushes through their command rings.

extern crate alloc;

use alloc::vec::Vec;

use super::domain::{attach, create_domain, destroy_domain, map_range};
use super::error::AmdViError;
use crate::arch::x86_64::acpi::parser::other::amd_reserved_for;
use crate::arch::x86_64::acpi::parser::other::ivmd::Reserved;

/// How many of `ids` were given reserved memory; the rest stay blocked.
pub(super) fn give_reserved(ids: impl Iterator<Item = u16>) -> usize {
    let mut made: Vec<(Vec<Reserved>, u16)> = Vec::new();
    let mut given = 0;
    for id in ids {
        let ranges: Vec<Reserved> = amd_reserved_for(id).into_iter().collect();
        if ranges.is_empty() {
            continue;
        }
        let domain = match made.iter().find(|(r, _)| *r == ranges) {
            Some(&(_, d)) => d,
            None => match domain_holding(&ranges) {
                Ok(d) => {
                    made.push((ranges, d));
                    d
                }
                Err(e) => {
                    crate::log::info!("[amd-vi] device {} reserved memory not mapped: {:?}", id, e);
                    continue;
                }
            },
        };
        if attach(domain, id).is_ok() {
            given += 1;
        }
    }
    given
}

fn domain_holding(ranges: &[Reserved]) -> Result<u16, AmdViError> {
    let domain = create_domain()?;
    for r in ranges {
        let size = (r.end - r.start) as usize;
        if let Err(e) = map_range(domain, r.start, r.start, size, (r.read, r.write)) {
            let _ = destroy_domain(domain);
            return Err(e);
        }
    }
    Ok(domain)
}
