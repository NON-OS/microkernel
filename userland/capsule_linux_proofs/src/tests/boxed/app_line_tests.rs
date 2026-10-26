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

//! The `[BOX-APP]` line names the program by the policy its receipt carries:
//! the receipt verifier rebuilds slot 0 from exactly these bytes, so a line
//! naming any other policy would ask for a receipt that never verifies.

use crate::app::identity::AppIdentity;
use crate::boxed::app_line::{app_line, policy_of};
use crate::boxed::hash::hex;
use crate::boxed::seal_slot::slot0;
use nonos_linux_policy::{app_id, decode, policy_hash, unlisted, LinuxAppPolicy};

fn published() -> AppIdentity {
    AppIdentity::of(LinuxAppPolicy { mask: 3, listing_id: [1; 32], ..unlisted([7; 32]) })
}

#[test]
fn a_published_program_is_named_by_its_own_policy() {
    let app = published();
    let p = policy_of(&app.app_id, &app.policy);
    assert_eq!(p, app.policy.encode());
    let back = decode(&p).expect("one policy");
    assert_eq!(app_id(&back.program_blake3, &policy_hash(&p)), app.app_id);
}

#[test]
fn the_built_in_busybox_is_named_by_program_zero() {
    let app = AppIdentity::built_in([9; 32]);
    let p = policy_of(&app.app_id, &app.policy);
    assert_eq!(p, unlisted([0; 32]).encode());
    assert_ne!(p, app.policy.encode());
}

#[test]
fn the_binding_the_verifier_rebuilds_is_the_one_sealed() {
    let app = published();
    let p = policy_of(&app.app_id, &app.policy);
    let back = decode(&p).expect("one policy");
    let rebuilt = app_id(&back.program_blake3, &policy_hash(&p));
    let h = [5; 32];
    assert_eq!(slot0(&rebuilt, Some(&h)), slot0(&app.app_id, Some(&h)));
}

#[test]
fn the_line_is_versioned_and_carries_every_byte() {
    let p = published().policy.encode();
    let line = app_line(&p);
    assert_eq!(line, format!("[BOX-APP] v=1 policy={}\n", hex(&p)));
    assert_eq!(line.len(), "[BOX-APP] v=1 policy=\n".len() + 512);
}
