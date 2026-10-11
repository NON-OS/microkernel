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

//! The per-minute bucket, and the key file: one line per answer,
//! "unix keyCommit cipherRoot buyer_pk K", pruned past OFFER_TTL + MAX_DELIVERY.

use super::super::bucket::Bucket;
use super::super::keys::{prune, record, Kept, KEEP};

#[test]
fn the_bucket_takes_per_minute_then_refills_evenly() {
    let mut b = Bucket::new(30, 0);
    assert_eq!((0..40).filter(|_| b.take(0)).count(), 30, "a minute's worth at once");
    assert!(!b.take(1_999), "less than one token back");
    assert!(b.take(2_000), "one back every two seconds at 30 a minute");
    assert!(!b.take(2_000));
    let mut idle = Bucket::new(30, 0);
    (0..30).for_each(|_| _ = idle.take(0));
    assert_eq!((0..40).filter(|_| idle.take(3_600_000)).count(), 30, "never past its cap");
}

fn kept(w: &[[u8; 32]; 4]) -> Kept<'_> {
    Kept { key_commit: &w[0], cipher_root: &w[1], buyer_pk: &w[2], k: &w[3] }
}

#[test]
fn a_key_line_holds_what_deliver_checks_and_old_lines_go() {
    let path = std::env::temp_dir().join(format!("escrow-keys-{}", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let w = |b: u8| [[b; 32], [b + 1; 32], [b + 2; 32], [b + 3; 32]];
    let (a, z) = (w(0x10), w(0x40));
    record(&path, 10, &kept(&a)).unwrap();
    record(&path, 10 + KEEP + 5, &kept(&z)).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let first: Vec<&str> = text.lines().next().unwrap().split(' ').collect();
    assert_eq!(
        first,
        ["10", &"10".repeat(32), &"11".repeat(32), &"12".repeat(32), &"13".repeat(32)]
    );
    assert_eq!(prune(&path, 10 + KEEP + 5).unwrap(), 1, "the first is past its life");
    let left = std::fs::read_to_string(&path).unwrap();
    assert!(left.lines().count() == 1 && left.contains(&"43".repeat(32)));
    let _ = std::fs::remove_file(&path);
}
