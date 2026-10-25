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
//! A broker loads only for someone: a load from it naming no attested
//! parent is refused, even for a child within the broker's own set, so it
//! never lends what it holds. And the bound's installed set is the verified
//! spawn's own (`verify/caps_bits.rs`), over many masks.

use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::error::LoadError;
use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::load::bound::{check, installed};
use crate::security::capsule_manifest::CapsuleManifest;
use crate::spawn_caps::install_caps;
use crate::stand_in::{said, world, AttestedParent};

const APP: u64 = 0x1 | 0x8 | 0x10;
const BROKER: u64 = 1 << 23;

fn plain() -> CapsuleManifest {
    CapsuleManifest { required_caps: APP, optional_caps: 0 }
}

#[test]
fn a_broker_naming_no_parent_is_refused_even_within_itself() {
    world(Some(2), &[(2, APP | BROKER), (9, APP)]);
    assert_eq!(check(&plain(), 0, None, "app"), Err(LoadError::BeyondParent));
    assert_eq!(said(), "[CAP-DENY] load: a broker named no attested parent\n");
    world(Some(2), &[(2, APP | BROKER), (9, APP)]);
    assert_eq!(check(&plain(), 0, Some(AttestedParent(9)), "app"), Ok(()));
    assert_eq!(said(), "");
}

#[test]
fn a_broker_naming_itself_is_refused() {
    world(Some(2), &[(2, APP | BROKER)]);
    assert_eq!(check(&plain(), 0, Some(AttestedParent(2)), "app"), Err(LoadError::BeyondParent));
    assert_eq!(said(), "[CAP-DENY] load: a broker named no attested parent\n");
}

#[test]
fn a_caller_holding_no_broker_may_load_for_itself() {
    world(Some(7), &[(7, APP)]);
    assert_eq!(check(&plain(), 0, None, "app"), Ok(()));
}

#[test]
fn installed_is_the_verified_spawn_s_install_caps() {
    let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
    for _ in 0..20_000 {
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        let (r, o, g) = (next(), next(), next());
        let m = CapsuleManifest { required_caps: r, optional_caps: o };
        assert_eq!(installed(&m, g), install_caps(r, o, g));
    }
}
