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

//! The bound as the loader applies it: against the attributed parent when
//! there is one, else the caller; refused with one serial line; and refused
//! outright when the caller or the parent is not a live process.

use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::error::LoadError;
use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::load::bound::check;
use crate::security::capsule_manifest::CapsuleManifest;
use crate::stand_in::{said, world, AttestedParent};

const APP: u64 = 0x1 | 0x8 | 0x10;
const NET: u64 = 1 << 2;
const INSTALLER: u64 = APP | 0x40 | (1 << 23);

fn net_app() -> CapsuleManifest {
    CapsuleManifest { required_caps: APP | NET, optional_caps: 0 }
}

#[test]
fn a_caller_without_the_bit_is_refused_and_it_is_said() {
    world(Some(7), &[(7, APP)]);
    assert_eq!(check(&net_app(), 0, None, "web"), Err(LoadError::BeyondParent));
    assert_eq!(said(), "[CAP-DENY] load: caller pid=7 parent pid=7 child=web missing caps 0x4\n");
    world(Some(7), &[(7, APP | NET)]);
    assert_eq!(check(&net_app(), 0, None, "web"), Ok(()));
    assert_eq!(said(), "", "an allowed load says nothing");
}

#[test]
fn the_installer_is_bound_by_whoever_it_loads_for() {
    // The installer itself holds no Network: loading for itself, it is refused.
    world(Some(2), &[(2, INSTALLER), (9, APP), (11, APP | (1 << 24))]);
    assert_eq!(check(&net_app(), 0, None, "web"), Err(LoadError::BeyondParent));
    // For an app that lacks Network: refused, the deputy closed.
    let for_app = Some(AttestedParent(9));
    assert_eq!(check(&net_app(), 0, for_app, "web"), Err(LoadError::BeyondParent));
    assert!(said().contains("caller pid=2 parent pid=9"));
    // For a launcher: allowed.
    assert_eq!(check(&net_app(), 0, Some(AttestedParent(11)), "web"), Ok(()));
}

#[test]
fn an_unknown_caller_or_parent_is_refused() {
    world(None, &[(9, u64::MAX)]);
    assert_eq!(check(&net_app(), 0, None, "web"), Err(LoadError::BeyondParent));
    world(Some(2), &[(2, u64::MAX)]);
    let gone = Some(AttestedParent(40));
    assert_eq!(check(&net_app(), 0, gone, "web"), Err(LoadError::BeyondParent));
}
