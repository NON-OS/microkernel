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

//! P.2: version 2 round trips and sits where the plan puts it, and a field
//! its SERVE_ONE bit does not grant moves no version 1 byte. Every committed
//! policy keeping its hash is held in capsule_linux_proofs, which reads the
//! market catalogue without making it an input of every capsule.

use super::sample::{policy, served};
use crate::{decode, LinuxAppPolicy};

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
fn a_field_the_bit_does_not_grant_is_not_written() {
    let unserved = LinuxAppPolicy { serve: served().serve, ..policy() };
    assert_eq!(unserved.encode(), policy().encode());
}
