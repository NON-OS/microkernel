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

//! Reading one error status block: its address from the source's Error
//! Status Address, the block's header for its length, then the whole
//! block; every entry logged and counted, and the block handed back to
//! firmware (ACPI 6.5, 18.3.2.7-8): its status cleared, and for GHESv2 the
//! Read Ack register written.

use super::phys::Phys;
use super::say::say;
use crate::sys::hw_error::hw_error;
use nonos_aml::apei::{parse_block, Ghes};

const HEADER: u64 = 20;
const SYSTEM_MEMORY: u8 = 0;

/// The block at `at`, at most `max` bytes; logged under `prefix`. Returns
/// the records read.
pub fn read_block(phys: &mut Phys, at: u64, max: u64, prefix: &[u8]) -> usize {
    let Some(head) = phys.bytes(at, HEADER) else { return 0 };
    let data = u64::from(u32::from_le_bytes([head[12], head[13], head[14], head[15]]));
    let Some(b) =
        phys.bytes(at, (HEADER + data).min(max.max(HEADER))).and_then(|b| parse_block(&b))
    else {
        return 0;
    };
    if b.is_empty() {
        return 0;
    }
    for e in &b.entries {
        say(prefix, e);
        let _ = hw_error(e.severity);
    }
    b.entries.len()
}

/// One GHES source: nothing when its block holds no new error.
pub fn read_source(phys: &mut Phys, g: &Ghes, prefix: &[u8]) -> usize {
    if g.status_address.space != SYSTEM_MEMORY {
        return 0;
    }
    let Some(block) = phys.read64(g.status_address.address).filter(|a| *a != 0) else { return 0 };
    let n = read_block(phys, block, u64::from(g.block_len), prefix);
    if n == 0 {
        return 0;
    }
    phys.write(block, 4, 0);
    if let Some(ack) = g.ack.filter(|a| a.register.space == SYSTEM_MEMORY) {
        let old = phys.read64(ack.register.address).unwrap_or(0);
        phys.write(ack.register.address, 8, (old & ack.preserve) | ack.write);
    }
    n
}
