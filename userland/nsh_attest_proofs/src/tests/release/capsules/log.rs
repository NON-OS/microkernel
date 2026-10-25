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

//! What PCR 14 may hold: capsule events alone, each 65 bytes, its digest
//! its body's SHA-256, its kind one of the four, and the path-only tag its
//! verifier exactly when it is path-only; and no capsule event in any other
//! PCR. Anything else is refused as a log the client cannot read.

use nsh_attest::{LogFault, Refusal};

use super::super::super::refused;
use crate::build::tcg::EV_APP;
use crate::build::tcg_capsule::{capsule_body, path_only_tag, EV_NONOS_CAPSULE};
use crate::build::tcg_capsule::{LOCAL_BUILD, PATH_ONLY, PROOF_VERIFIED};
use crate::build::tcg_event::digest;
use crate::build::tcg_log::Log;
use crate::build::Spec;

fn with(f: fn(&mut Log)) -> Refusal {
    let mut s = Spec::default();
    f(&mut s.log);
    refused(s)
}

#[test]
fn an_event_pcr_14_cannot_hold_is_refused() {
    let bad: [fn(&mut Log); 8] = [
        |l| _ = l.ext(14, EV_APP, digest(b"not a capsule")),
        |l| _ = l.note(14, EV_NONOS_CAPSULE, &[1; 64]),
        |l| _ = l.note(14, EV_NONOS_CAPSULE, &capsule_body([1; 32], [2; 32], 5)),
        |l| _ = l.note(5, EV_NONOS_CAPSULE, &capsule_body([1; 32], [2; 32], PROOF_VERIFIED)),
        |l| _ = l.capsule([1; 32], digest(b"nonos capsule path-only"), PROOF_VERIFIED),
        |l| _ = l.capsule([1; 32], [2; 32], PATH_ONLY),
        |l| _ = l.capsule([1; 32], path_only_tag(), LOCAL_BUILD),
        |l| {
            _ = l.note(
                14,
                EV_NONOS_CAPSULE,
                &[&capsule_body([1; 32], [2; 32], 1)[..], &[0]].concat(),
            )
        },
    ];
    for f in bad {
        assert_eq!(with(f), Refusal::Log(LogFault::Capsule));
    }
}

#[test]
fn a_body_changed_under_its_digest_is_refused() {
    // The last capsule's body: a certificate byte, then its kind as 1 to 3.
    for (back, to) in [(65, 0x77), (1, 3)] {
        let mut s = Spec::default();
        let at = s.log.bytes.len() - back;
        s.log.bytes[at] = to;
        assert_eq!(refused(s), Refusal::Log(LogFault::Capsule), "{back}");
    }
}
