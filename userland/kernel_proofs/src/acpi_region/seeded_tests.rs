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

//! F-102: the region rule over seeded machines, by F-98's method. Each seed
//! draws a memory map of every kind, windows and ports other capsules hold,
//! the kernel's hardware (MSI-X tables among it) and claimed functions,
//! then asks for ranges that straddle them. A grant never reaches a page or
//! port that, alone, would be refused; a write never reaches a held function.

use super::machine::Machine;
use super::rule::{io, memory, pci_config, Facts, Refusal, LEGACY_PORTS};
use super::seeded_gen::{machine, near, Rng};

fn page_ok(m: &Machine, page: u64) -> bool {
    let mut ok = true;
    m.mem_kinds(page, page + 0x1000, &mut |k| ok &= k.grantable());
    ok && m.protected(page, page + 0x1000).is_none() && !m.mmio_claimed(page, page + 0x1000)
}

fn port_ok(m: &Machine, p: u64) -> bool {
    !LEGACY_PORTS.iter().any(|(lo, hi, _)| *lo <= p && p < *hi) && !m.ports_claimed(p as u16, 1)
}

#[test]
fn seeded_machines_grant_only_what_each_page_and_port_allows() {
    let mut n = [[0u32; 2]; 2]; // memory, ports; refused, granted
    for seed in 1..=600u64 {
        let r = &mut Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let m = machine(r);
        for _ in 0..40 {
            let (base, end) = near(r, &m);
            let whole = (base..end).step_by(0x1000).all(|p| page_ok(&m, p));
            let ok = memory(base, end - base, &m).is_ok();
            assert_eq!(ok, whole, "seed {seed}: memory {base:#x}..{end:#x}");
            n[0][usize::from(ok)] += 1;
            let (lo, hi) = r.span(0x1000, 1);
            let len = (hi - lo).min(4);
            let lo = if r.below(2) == 0 { lo } else { LEGACY_PORTS[r.below(14) as usize].0 };
            let hi = lo + len;
            let ok = io(lo, hi - lo, &m).is_ok();
            assert_eq!(ok, (lo..hi).all(|p| port_ok(&m, p)), "seed {seed}: ports {lo:#x}..{hi:#x}");
            let bdf = (0, r.below(4) as u8, r.below(32) as u8, r.below(8) as u8);
            let held = m.pci.contains(&bdf);
            assert_eq!(
                pci_config(bdf, true, &m),
                if held { Err(Refusal::Claimed) } else { Ok(()) }
            );
            assert_eq!(pci_config(bdf, false, &m), Ok(()), "reads are not narrowed");
            n[1][usize::from(ok)] += 1;
        }
    }
    assert!(n.iter().flatten().all(|c| *c > 4000), "both answers, both kinds: {n:?}");
}
