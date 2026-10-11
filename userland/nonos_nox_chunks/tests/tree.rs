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

#[path = "common/k.rs"]
mod k;

use k::{kec, K};
use nonos_nox_chunks::{leaf, proof, root, verify, PLAIN_TREE};

/// The reference tree: plan/22's rules written out recursively.
fn sorted(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    kec(&[&lo, &hi])
}

/// Level by level, the last of an odd level carried up as is.
fn reference_root(leaves: &[[u8; 32]]) -> Option<[u8; 32]> {
    match leaves.len() {
        0 => None,
        1 => Some(leaves[0]),
        _ => {
            let next: Vec<[u8; 32]> = leaves
                .chunks(2)
                .map(|p| if p.len() == 2 { sorted(p[0], p[1]) } else { p[0] })
                .collect();
            reference_root(&next)
        }
    }
}

fn leaves(n: usize) -> Vec<[u8; 32]> {
    (0..n).map(|i| leaf(&K, PLAIN_TREE, i as u64, &[i as u8; 32])).collect()
}

#[test]
fn roots_match_the_reference_for_every_shape() {
    assert_eq!(root(&K, &[]), None);
    for n in 1..=33 {
        assert_eq!(root(&K, &leaves(n)), reference_root(&leaves(n)), "{n} leaves");
    }
    let l = leaves(3);
    assert_eq!(root(&K, &l), Some(sorted(sorted(l[0], l[1]), l[2])));
}

#[test]
fn every_leaf_proves_into_its_root_and_nothing_else_does() {
    for n in 1..=17 {
        let l = leaves(n);
        let r = root(&K, &l).unwrap();
        for i in 0..n {
            let p = proof(&K, &l, i).unwrap();
            assert!(verify(&K, &l[i], &p, &r), "{n} leaves, leaf {i}");
            let mut forged = l[i];
            forged[0] ^= 1;
            assert!(!verify(&K, &forged, &p, &r));
        }
        assert_eq!(proof(&K, &l, n), None);
    }
}
