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

use super::dmar_andd::parse_andd;
use super::dmar_hostile::{table, Seed};
use super::dmar_rmrr::{parse_rmrr, MAX_RMRR_SCOPES, RMRR_TYPE};
use super::dmar_scope_walk::walk_scopes;
use super::dmar_walk::{walk_structures, FIRST_STRUCTURE};
use super::scope::{Scope, MAX_SCOPES};

#[test]
fn hostile_dmars_are_walked_whole_or_cut_and_nothing_reads_past_them() {
    let (mut s, mut cut, mut rmrrs) = (Seed(0x9E37_79B9_7F4A_7C15), 0, 0);
    for _ in 0..3000 {
        let t = table(&mut s);
        let mut at = FIRST_STRUCTURE;
        let walked = walk_structures(&t, |kind, bytes| {
            // Whole, in order, its own length and kind, inside the table.
            assert_eq!(bytes.as_ptr() as usize - t.as_ptr() as usize, at);
            assert!(bytes.len() >= 4 && at + bytes.len() <= t.len());
            assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]) as usize, bytes.len());
            assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), kind);
            at += bytes.len();
            if let Some(r) = parse_rmrr(bytes).filter(|_| kind == RMRR_TYPE) {
                rmrrs += 1;
                assert!(r.count as usize <= MAX_RMRR_SCOPES && r.base <= r.limit);
            }
            let _ = parse_andd(bytes);
            let mut scopes = [Scope::EMPTY; MAX_SCOPES];
            assert!(walk_scopes(bytes, 16, &mut scopes).0 as usize <= MAX_SCOPES);
        });
        // A walk not cut short ends where too little, or only padding, is left.
        assert!(walked.cut_short || t.len() < at + 4 || t[at..].iter().all(|&b| b == 0));
        cut += walked.cut_short as u32;
    }
    assert!(cut > 300 && rmrrs > 300, "{cut} cut short, {rmrrs} RMRRs parsed");
}
