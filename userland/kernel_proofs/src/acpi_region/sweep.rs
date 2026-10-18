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

//! The rule against an oracle that looks at one page or port at a time: over
//! many ranges, nothing granted contains a byte the oracle would refuse.

use super::machine::{pc, Machine};
use super::rule::{io, memory, Facts, LEGACY_PORTS};

/// Whether one page, alone, is firmware's or free device space.
fn page_ok(m: &Machine, page: u64) -> bool {
    let end = page + 0x1000;
    let mut ok = true;
    m.mem_kinds(page, end, &mut |k| ok &= k.grantable());
    ok && m.protected(page, end).is_none() && !m.mmio_claimed(page, end)
}

fn port_ok(m: &Machine, port: u64) -> bool {
    let legacy = LEGACY_PORTS.iter().any(|(lo, hi, _)| *lo <= port && port < *hi);
    !legacy && !m.ports_claimed(port as u16, 1)
}

/// A fixed-seed generator, so a failure names the same range every run.
fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *seed >> 16
}

#[test]
fn a_granted_memory_range_is_granted_page_by_page() {
    let m = pc();
    let mut seed = 7;
    let bases =
        [0, 0xF_0000, 0x7FD0_0000, 0x8000_0000, 0xDFFF_0000, 0xFEAF_0000, 0xFEBF_0000, 0xFEDF_0000];
    for _ in 0..20_000 {
        let base = (bases[(next(&mut seed) % 8) as usize] + next(&mut seed) % 0x40_0000) & !0xFFF;
        let len = 0x1000 * (1 + next(&mut seed) % 64);
        let whole = (base..base + len).step_by(0x1000).all(|p| page_ok(&m, p));
        assert_eq!(memory(base, len, &m).is_ok(), whole, "{base:#x}+{len:#x}");
    }
}

#[test]
fn a_granted_port_range_is_granted_port_by_port() {
    let m = pc();
    let mut seed = 11;
    for _ in 0..20_000 {
        let base = next(&mut seed) % 0x1000;
        let len = 1 + next(&mut seed) % 16;
        let whole = (base..base + len).all(|p| port_ok(&m, p));
        assert_eq!(io(base, len, &m).is_ok(), whole, "{base:#x}+{len:#x}");
    }
}
