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

//! I-C6: every proved app noted to slot 0 of the seal, between its proof
//! and its start.

use crate::app::identity::AppIdentity;
use crate::boxed::preset::parse;
use nonos_linux_policy::mask::KNOWN;
use nonos_linux_policy::{unlisted, LinuxAppPolicy};

/*
 * I-C6: slot 0 is noted once the program is proved and before its guest
 * starts; the kernel takes it no later (src/process/foreign/seal/sys_note.rs),
 * and a family never noted is dropped at its end.
 */
#[test]
fn the_app_is_noted_to_the_seal_between_its_proof_and_its_start() {
    let start = include_str!("../../../../capsule_linux/src/linux/start_guest.rs");
    let at = |needle: &str| start.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    let (proved, noted, started) = (
        at("prove(path, bytes"),
        at("seal_app(&guest.app.app_id)"),
        at("mk_foreign_start(guest.pid"),
    );
    assert!(proved < noted && noted < started);
    let note = include_str!("../../../../capsule_linux/src/linux/seal_note.rs");
    assert!(note.contains("mk_seal_note(nonos_libc::SEAL_APP_ID, &note)"));
    assert!(note.contains("super::seal_slot::slot0(app_id, super::boxed::armed_hash().as_ref())"));
    let spawn = include_str!("../../../../capsule_linux/src/linux/start.rs");
    assert!(spawn.find("mk_foreign_spawn").unwrap() < spawn.find("start(&mut guest").unwrap());
}

#[test]
fn outside_a_box_every_proved_app_is_noted_and_the_built_in_busybox_is_not() {
    use crate::boxed::seal_slot::slot0;
    assert_eq!(slot0(&[0; 32], None), None);
    let mut last = [0; 32];
    last[31] = 1;
    for app in [[1; 32], last] {
        assert_eq!(slot0(&app, None), Some(app), "the app_id itself");
    }
}

// The box narrows what the app may do, never which app the seal is told it is.
#[test]
fn the_box_keeps_the_programs_identity_and_calls() {
    let own = AppIdentity::of(LinuxAppPolicy { mask: KNOWN, ..unlisted([7; 32]) });
    let p = parse(b"offline").unwrap();
    let boxed = own.boxed(p.mask, Vec::leak(p.dests.clone()));
    assert_eq!(
        (boxed.app_id, boxed.policy_hash, boxed.calls),
        (own.app_id, own.policy_hash, own.calls)
    );
}
