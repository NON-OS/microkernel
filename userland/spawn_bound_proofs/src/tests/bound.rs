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

//! The load bound, case by case: a child within its parent is started; one
//! bit beyond it is refused, with one serial line; a launcher may start
//! beyond itself; the parent is the attributed one when the installer names
//! it; and an unknown parent or caller is refused.

use crate::capabilities::Capability as C;
use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::load::bound::{installed, missing};
use crate::security::capsule_manifest::CapsuleManifest;

const APP: u64 = 0x1 | 0x8 | 0x10; // CoreExec | IPC | Memory
const NET: u64 = 1 << 2;

fn manifest(required: u64, optional: u64) -> CapsuleManifest {
    CapsuleManifest { required_caps: required, optional_caps: optional }
}

#[test]
fn the_bits_are_the_kernel_s() {
    assert_eq!(C::Network.bit(), NET);
    assert_eq!(C::CoreExec.bit() | C::IPC.bit() | C::Memory.bit(), APP);
    assert_eq!(C::SpawnWindow.bit(), 1 << 24);
}

#[test]
fn within_the_parent_is_allowed_and_one_bit_more_is_not() {
    assert_eq!(missing(APP, APP), 0);
    assert_eq!(missing(C::IPC.bit(), APP), 0);
    assert_eq!(missing(APP | NET, APP), NET, "exactly the missing bit");
    for bit in 0..64 {
        let extra = 1u64 << bit;
        let want = if APP & extra != 0 { 0 } else { extra };
        assert_eq!(missing(APP | extra, APP), want, "bit {bit}");
    }
}

#[test]
fn a_launcher_may_start_beyond_itself_and_only_a_launcher() {
    for right in [C::SpawnWindow, C::Admin] {
        assert_eq!(missing(u64::MAX, APP | right.bit()), 0, "{right:?}");
    }
    for not_a_launcher in [C::SpawnBroker, C::ProcessControl, C::Network] {
        let parent = APP | not_a_launcher.bit();
        assert_ne!(missing(u64::MAX, parent), 0, "{not_a_launcher:?}");
    }
}

#[test]
fn installed_is_required_and_the_granted_optional() {
    let m = manifest(APP, NET | C::Crypto.bit());
    assert_eq!(installed(&m, 0), APP);
    assert_eq!(installed(&m, NET), APP | NET);
    assert_eq!(installed(&m, u64::MAX), APP | NET | C::Crypto.bit());
}
