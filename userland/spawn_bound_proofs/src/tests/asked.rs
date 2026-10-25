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

//! A load asks only for what its manifest offers: a sender naming one bit
//! more is refused with one serial line, and anything within passes as named.

use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::error::LoadError;
use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::load::asked::{
    beyond, within_manifest,
};
use crate::security::capsule_manifest::CapsuleManifest;
use crate::stand_in::{said, world};

const REQUIRED: u64 = 0x1 | 0x8 | 0x10;
const OPTIONAL: u64 = 1 << 8;

fn app() -> CapsuleManifest {
    CapsuleManifest { required_caps: REQUIRED, optional_caps: OPTIONAL }
}

#[test]
fn a_sender_asking_for_one_bit_more_is_refused_and_it_is_said() {
    world(Some(5), &[(5, REQUIRED)]);
    assert_eq!(within_manifest(&app(), REQUIRED | (1 << 9), "web"), Err(LoadError::BeyondParent));
    assert_eq!(said(), "[CAP-DENY] load: caller pid=5 child=web asked beyond its manifest 0x200\n");
    world(Some(5), &[(5, REQUIRED)]);
    assert_eq!(within_manifest(&app(), u64::MAX, "web"), Err(LoadError::BeyondParent));
    assert!(said().contains(&format!("{:#x}", !(REQUIRED | OPTIONAL))));
}

#[test]
fn within_the_manifest_passes_as_named_and_says_nothing() {
    for requested in [0, REQUIRED, OPTIONAL, REQUIRED | OPTIONAL, 0x8] {
        world(Some(5), &[(5, REQUIRED)]);
        assert_eq!(within_manifest(&app(), requested, "web"), Ok(requested));
        assert_eq!(said(), "");
    }
}

#[test]
fn beyond_is_every_bit_the_manifest_does_not_name_over_every_mask() {
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..20_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let (req, opt) = (x & 0x00FF_00FF, (x >> 8) & 0xFF00 & !(x & 0x00FF_00FF));
        let m = CapsuleManifest { required_caps: req, optional_caps: opt };
        let asked = x.rotate_left(29);
        for bit in 0..64 {
            let b = 1u64 << bit;
            let offered = (req | opt) & b != 0;
            assert_eq!(beyond(&m, asked) & b != 0, asked & b != 0 && !offered);
        }
    }
}
