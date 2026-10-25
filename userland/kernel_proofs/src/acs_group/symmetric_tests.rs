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

//! The group relation both ways (H-03, port_only.rs `blocked_by`): whether
//! a claim is refused never depends on which of two devices is claimed
//! first, on any generated machine (symmetric_topologies.rs).

use super::acs_tree::Bdf;
use super::member::Member;
use super::port_only::blocked_by;
use super::symmetric_groups::groups;
use super::symmetric_topologies::machine;

/// Whether claiming `d` (port-only `po_d`) is refused while another
/// capsule (port-only `po_e`) holds `e`.
pub(super) fn refused(
    groups: &[Vec<Member>],
    at: &[Bdf],
    (d, po_d): (usize, bool),
    (e, po_e): (usize, bool),
) -> bool {
    let peer = |id: u64| Some((at[id as usize], Some(&groups[id as usize][..])));
    blocked_by(d as u64, &groups[d], &[e as u64], (po_d, po_e), peer).is_some()
}

#[test]
fn on_generated_machines_no_refusal_depends_on_the_claim_order() {
    let mut pairs = 0;
    for seed in 0..400 {
        let m = machine(seed);
        let gs = groups(&m);
        let at: Vec<Bdf> = m.funcs.iter().map(|f| f.at).collect();
        for &d in &m.leaves {
            for &e in m.leaves.iter().filter(|&&e| e != d) {
                for (po_d, po_e) in [(false, false), (true, false), (false, true), (true, true)] {
                    let first = refused(&gs, &at, (d, po_d), (e, po_e));
                    let second = refused(&gs, &at, (e, po_e), (d, po_d));
                    assert_eq!(first, second, "seed {seed}: {:?} and {:?}", at[d], at[e]);
                    pairs += 1;
                }
            }
        }
    }
    assert!(pairs > 1_000, "only {pairs} pairs tried");
}

#[test]
fn generated_machines_hold_one_sided_groups_for_the_proof_to_bite() {
    let one_sided = (0..400).any(|seed| {
        let m = machine(seed);
        let gs = groups(&m);
        m.leaves
            .iter()
            .any(|&d| gs[d].iter().any(|x| !gs[x.id as usize].iter().any(|y| y.id == d as u64)))
    });
    assert!(one_sided, "every generated group is symmetric already; the proof proves nothing");
}
