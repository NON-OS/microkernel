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

//! The writer's walk of the log (napr/capsules.rs) takes exactly the PCR 14
//! capsules nsh_attest's replay finds, and a receipt whose capsules do not
//! replay to its quoted PCR 14, one missing, one added or one relabelled, is
//! refused as `capsules`.

use agentproof_receipt::napr::capsules::capsules_of;
use agentproof_receipt::napr::read::parse;
use agentproof_receipt::napr::write;
use agentproof_receipt::verify;
use nsh_attest::log::replay;

use crate::build::tcg_capsule::{path_only_tag, DEVELOPER, LOCAL_BUILD, PATH_ONLY};
use crate::build::tcg_log::Log;
use crate::fixture::Fixture;

#[test]
fn the_walk_takes_what_nsh_attests_replay_takes() {
    let mut log = Log::standard();
    for (i, k) in [1u8, PATH_ONLY, DEVELOPER, LOCAL_BUILD, 1].into_iter().enumerate() {
        let v = if k == PATH_ONLY { path_only_tag() } else { [0x40 + i as u8; 32] };
        log.capsule([i as u8; 32], v, k);
        log.note(5, 0x8000_0007, b"between");
    }
    let r = replay(&log.bytes).unwrap();
    let theirs: Vec<u8> = r
        .capsules
        .iter()
        .flat_map(|c| [&c.cert[..], &c.verifier[..], &[c.admitted as u8 + 1]].concat())
        .collect();
    assert_eq!(capsules_of(&log.bytes), Some(theirs));
    assert_eq!(capsules_of(&log.bytes[..log.bytes.len() - 1]), None, "a cut log");
}

#[test]
fn capsules_that_do_not_replay_are_refused() {
    let mut f = Fixture::new(b"strict");
    let (bytes, t) = (f.bytes(), f.trust());
    let r = parse(&bytes).unwrap();
    let c = r.capsules;
    let mut relabelled = c.to_vec();
    relabelled[64] = 3;
    for capsules in [&c[65..], &[c, &c[..65]].concat()[..], &relabelled[..]] {
        let b = write::finish(r.bound.to_vec(), r.doc, r.entries, capsules);
        assert_eq!(verify(&b, &t).unwrap_err().name(), "capsules");
    }
}
