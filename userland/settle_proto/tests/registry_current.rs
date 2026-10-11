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

use nonos_settle_proto::{current, Listed, LISTED_MAX};

fn l(listing: u8, release: u8, block: u64) -> Listed {
    Listed {
        listing_id: [listing; 32],
        release_id: [release; 32],
        release_hash: [block as u8; 32],
        bond_id: [7; 32],
        block,
        uri: b"https://h/r".to_vec(),
    }
}

fn ids(v: &[Listed]) -> Vec<(u8, u64)> {
    v.iter().map(|x| (x.release_id[0], x.block)).collect()
}

#[test]
fn only_the_listing_newest_first() {
    let p = [l(1, 1, 10), l(2, 9, 50), l(1, 2, 30), l(1, 3, 20)];
    assert_eq!(ids(&current(&[1; 32], &p, &[])), [(2, 30), (3, 20), (1, 10)]);
    assert!(current(&[5; 32], &p, &[]).is_empty());
}

#[test]
fn one_row_per_release_its_newest_published() {
    let p = [l(1, 1, 30), l(1, 1, 10), l(1, 1, 20)];
    let c = current(&[1; 32], &p, &[]);
    assert_eq!(ids(&c), [(1, 30)]);
    assert_eq!(c[0].release_hash, [30; 32]);
}

#[test]
fn a_withdrawn_release_is_gone_for_good() {
    let p = [l(1, 1, 10), l(1, 2, 20), l(1, 1, 40)];
    let w = [([1; 32], [1; 32])];
    assert_eq!(ids(&current(&[1; 32], &p, &w)), [(2, 20)]);
    let other_listing = [([2; 32], [2; 32])];
    assert_eq!(current(&[1; 32], &p, &other_listing).len(), 2);
}

#[test]
fn at_most_listed_max_rows() {
    let p: Vec<Listed> = (0..40).map(|i| l(1, i as u8, i)).collect();
    let c = current(&[1; 32], &p, &[]);
    assert_eq!(c.len(), LISTED_MAX);
    assert_eq!(c[0].block, 39);
    assert_eq!(c[LISTED_MAX - 1].block, 40 - LISTED_MAX as u64);
}
