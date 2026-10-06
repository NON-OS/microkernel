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

//! Bounded proofs that the readers every gate runs on untrusted bytes never
//! panic, and accept only the shapes their layouts name. `cargo kani` checks
//! every input of the stated sizes, not a sample.

use crate::leaf::Kind;
use crate::trailer::{parse, MAGIC};
use crate::v4::parse_v4;

/// Bytes enough for a v4 header, a short path and a short proof.
const V4_BYTES: usize = 48;

#[kani::proof]
fn parse_v4_never_panics_and_accepts_only_its_layout() {
    let t: [u8; V4_BYTES] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= V4_BYTES);
    let max_proof: usize = kani::any();
    let expect = if kani::any() { Kind::Capsule } else { Kind::Bootloader };
    if let Some(v) = parse_v4(&t[..len], expect, max_proof) {
        assert!(v.kind == expect);
        assert!(!v.proof.is_empty() && v.proof.len() <= max_proof);
        assert!(v.path.starts_with(&MAGIC));
        assert_eq!(13 + v.path.len() + 4 + v.proof.len(), len);
    }
}

/// A depth-2 v3 path: magic, depth, two siblings, one direction byte.
const V3_BYTES: usize = 9 + 2 * 32 + 1;

#[kani::proof]
#[kani::unwind(9)]
fn v3_path_never_panics_and_has_no_spare_direction_bits() {
    let t: [u8; V3_BYTES] = kani::any();
    let depth: usize = kani::any();
    kani::assume(depth <= 3);
    if let Some(p) = parse(&t, depth) {
        assert_eq!(depth, 2);
        assert_eq!(t[V3_BYTES - 1] >> 2, 0);
        assert!(p.right(0).is_some() && p.right(1).is_some());
    }
}
