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

//! PCR 14, replayed: every capsule a release host admitted was proof-verified
//! and recorded, and the quoted PCR 14 is what the log's events give. A
//! dropped, reordered or relabelled extend breaks the replay; a path-only
//! capsule logged as one is refused, and so is a host that recorded none.

use nsh_attest::{CapsuleFault, Refusal};

use super::super::{passes, refused};

mod log;
mod routes;
use crate::build::tcg_capsule::{path_only_tag, PATH_ONLY, PROOF_VERIFIED};
use crate::build::tcg_log::Log;
use crate::build::Spec;

/// The default host's log, with `caps` admitted in this order.
pub(super) fn log_of(caps: &[(u8, u8)]) -> Log {
    let mut l = Log::standard();
    for (i, kind) in caps {
        let verifier = if *kind == PATH_ONLY { path_only_tag() } else { [0xC0 + i; 32] };
        l.capsule([*i; 32], verifier, *kind);
    }
    l
}

/// A host whose log is `shown` while its TPM extended `extended`.
fn shown_not_extended(shown: &[(u8, u8)], extended: &[(u8, u8)]) -> Refusal {
    let pcr14 = log_of(extended).pcrs[14];
    refused(Spec { log: log_of(shown), pcr: vec![(14, pcr14)], ..Spec::default() })
}

#[test]
fn every_capsule_proof_verified_and_recorded_passes() {
    passes(Spec::default());
    passes(Spec { log: log_of(&[(7, PROOF_VERIFIED)]), ..Spec::default() });
}

#[test]
fn a_host_that_recorded_no_capsule_is_refused() {
    let s = Spec { log: Log::standard(), ..Spec::default() };
    assert_eq!(refused(s), Refusal::Pcr14(CapsuleFault::None));
}

#[test]
fn a_dropped_or_reordered_extend_breaks_the_replay() {
    let all = [(1, PROOF_VERIFIED), (2, PROOF_VERIFIED), (3, PROOF_VERIFIED)];
    assert_eq!(shown_not_extended(&all[..2], &all), Refusal::Replay(14));
    let swapped = [all[0], all[2], all[1]];
    assert_eq!(shown_not_extended(&swapped, &all), Refusal::Replay(14));
}

#[test]
fn a_path_only_capsule_logged_as_proof_verified_breaks_the_replay() {
    let was = [(1, PROOF_VERIFIED), (2, PATH_ONLY)];
    let claimed = [(1, PROOF_VERIFIED), (2, PROOF_VERIFIED)];
    assert_eq!(shown_not_extended(&claimed, &was), Refusal::Replay(14));
}
