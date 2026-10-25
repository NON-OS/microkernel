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
//! A whole receipt verifies, and says what it carries.
//! What a verified receipt says of the run: the personality's own BusyBox as
//! naming no program, and a learn run as not enforced.

use agentproof_policy::boxed::hash::box_hash;
use agentproof_policy::boxed::preset::parse;
use agentproof_receipt::seal_slot::slot0;
use agentproof_receipt::verify;

use crate::fixture::Fixture;

#[test]
fn a_busybox_run_in_a_box_verifies_as_the_built_in_program() {
    let mut f = Fixture::new(b"strict");
    let hash = box_hash(&parse(b"strict").unwrap());
    f.record.app_id = slot0(&[0; 32], Some(&hash)).unwrap();
    f.policy = nonos_linux_policy::unlisted([0; 32]).encode();
    let (bytes, trust) = (f.bytes(), f.trust());
    let v = verify(&bytes, &trust).unwrap();
    assert!(v.built_in && v.program.is_none(), "its note names no program, so neither does it");
    for at in [8, 40, 100, 255] {
        let mut g = Fixture::new(b"strict");
        g.record.app_id = f.record.app_id;
        g.policy = f.policy;
        g.policy[at] ^= 1; // a program claimed, or any other policy: not BusyBox's
        let (bytes, trust) = (g.bytes(), g.trust());
        assert!(verify(&bytes, &trust).is_err(), "policy byte {at}");
    }
}

#[test]
fn a_learn_run_verifies_as_not_enforced() {
    let mut f = Fixture::new(b"learn");
    let (bytes, trust) = (f.bytes(), f.trust());
    let v = verify(&bytes, &trust).unwrap();
    assert!(!v.enforced);
    for word in [&b"strict"[..], b"offline", b"allow:/tmp/o"] {
        let mut f = Fixture::new(word);
        let (bytes, trust) = (f.bytes(), f.trust());
        assert!(verify(&bytes, &trust).unwrap().enforced, "{word:?}");
    }
}
