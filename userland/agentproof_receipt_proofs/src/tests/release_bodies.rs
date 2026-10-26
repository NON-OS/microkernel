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

//! A capsule's body as the verifier holds it: a path-only kind only with the
//! path-only tag, and no other kind with it; the writer's walk takes PCR 14's
//! capsule events alone; and a capsules field is whole 65-byte bodies.

use agentproof_receipt::napr::capsules::capsules_of;
use agentproof_receipt::napr::read::{parse, Fault};
use agentproof_receipt::napr::write;
use agentproof_receipt::verify;

use crate::build::tcg_capsule::{capsule_body, path_only_tag, EV_NONOS_CAPSULE, PATH_ONLY};
use crate::build::tcg_log::Log;
use crate::fixture::Fixture;

#[test]
fn a_path_only_kind_needs_the_path_only_tag_and_no_other_kind_has_it() {
    for (verifier, kind) in [([9; 32], PATH_ONLY), (path_only_tag(), 1)] {
        let mut f = Fixture::new(b"strict");
        let mut log = Log::standard();
        log.capsule([1; 32], verifier, kind);
        f.spec.log = log;
        let (bytes, t) = (f.bytes(), f.trust());
        assert_eq!(verify(&bytes, &t).unwrap_err().name(), "capsules");
    }
}

#[test]
fn the_walk_takes_pcr_14_alone() {
    let mut log = Log::standard().with_capsules();
    let ours = capsules_of(&log.bytes).unwrap();
    log.note(5, EV_NONOS_CAPSULE, &capsule_body([7; 32], [8; 32], 1));
    assert_eq!(capsules_of(&log.bytes), Some(ours));
}

#[test]
fn a_capsules_field_is_whole_bodies() {
    let mut f = Fixture::new(b"strict");
    let bytes = f.bytes();
    let r = parse(&bytes).unwrap();
    let b = write::finish(r.bound.to_vec(), r.doc, r.entries, &r.capsules[..64]);
    assert_eq!(parse(&b), Err(Fault::Field("capsules")));
}
