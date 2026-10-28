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

//! P.2: version 2 round trips and sits where the plan puts it, and moves no
//! version 1 byte: every committed policy encodes to itself, and so does it
//! with a field its SERVE_ONE bit does not grant.

use super::sample::served;
use crate::{decode, policy_hash, LinuxAppPolicy, ServeOne, LEN};

/// The market catalogue the operator publishes, each release's guest_policy.
const CATALOGUE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/market/catalogue");
const KEY: &str = "\"guest_policy\": \"";

#[test]
fn a_served_policy_round_trips_as_version_2() {
    let (p, b) = (served(), served().encode());
    assert_eq!(decode(&b), Ok(p));
    assert_eq!(&b[8..10], &[2, 0]);
    assert_eq!(&b[217..219], &5225u16.to_le_bytes());
    assert_eq!(&b[219..230], b"app.simplex");
    assert!(b[230..].iter().all(|&x| x == 0));
    let s = p.serve_one().unwrap();
    assert_eq!((s.port, s.name(), s.name_len), (5225, "app.simplex", 11));
}

#[test]
fn every_committed_policy_keeps_its_bytes_and_hash() {
    let serve = ServeOne::new(5225, b"app.simplex");
    let mut seen = 0;
    for file in std::fs::read_dir(CATALOGUE).unwrap() {
        let text = std::fs::read_to_string(file.unwrap().path()).unwrap();
        for hex in text.split(KEY).skip(1) {
            let b: [u8; LEN] =
                core::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap());
            let p = decode(&b).unwrap();
            assert_eq!((p.encode(), p.serve_one()), (b, None));
            let unserved = LinuxAppPolicy { serve, ..p };
            assert_eq!(unserved.serve_one(), None);
            assert_eq!(policy_hash(&unserved.encode()), policy_hash(&b));
            seen += 1;
        }
    }
    assert_ne!(seen, 0, "no guest_policy under {CATALOGUE}");
}
