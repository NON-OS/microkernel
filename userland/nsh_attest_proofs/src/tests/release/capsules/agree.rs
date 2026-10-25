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

//! Steps 5 and 7 state one policy: a release host runs vendor-root
//! capsules only, for the whole boot. A capsule that is not the vendor's is
//! refused by PCR 14 for its kind and by the registry for its authority,
//! with or without Network, and a kind 3 or 4 capsule cannot pass as kind 1.

use nsh_attest::{CapsuleFault, Refusal};

use super::super::super::{passes, refused};
use super::log_of;
use crate::build::parts::entry_by;
use crate::build::tcg_capsule::{DEVELOPER, LOCAL_BUILD, PATH_ONLY, PROOF_VERIFIED};
use crate::build::Spec;

/// The default host with one more capsule: its PCR 14 kind, and the
/// authority byte and capabilities of its registry entry.
fn host(kind: u8, authority: u8, caps: u64) -> Spec {
    let mut s = Spec { log: log_of(&[(1, PROOF_VERIFIED), (2, kind)]), ..Spec::default() };
    s.entries.push(entry_by(9, [0x99; 32], caps, authority));
    s
}

#[test]
fn the_two_steps_agree_on_every_kind() {
    passes(host(PROOF_VERIFIED, 0, 0));
    let kinds = [
        (PATH_ONLY, CapsuleFault::PathOnly),
        (DEVELOPER, CapsuleFault::Developer),
        (LOCAL_BUILD, CapsuleFault::LocalBuild),
    ];
    for (kind, fault) in kinds {
        for caps in [0, 1 << 3, nsh_attest::steps::NETWORK] {
            // Step 5 refuses it first, whatever the registry says.
            assert_eq!(refused(host(kind, 1, caps)), Refusal::Pcr14(fault));
            assert_eq!(refused(host(kind, 0, caps)), Refusal::Pcr14(fault));
        }
    }
    // Step 7 refuses a non-vendor entry even when PCR 14 is all kind 1.
    for (authority, caps) in [(1, 0), (2, 1 << 3), (255, 0), (1, nsh_attest::steps::NETWORK)] {
        assert_eq!(refused(host(PROOF_VERIFIED, authority, caps)), Refusal::NotVendor);
    }
}

#[test]
fn a_kind_3_or_4_capsule_relabelled_as_kind_1_breaks_the_replay() {
    for kind in [DEVELOPER, LOCAL_BUILD] {
        let extended = log_of(&[(1, PROOF_VERIFIED), (2, kind)]);
        let shown = log_of(&[(1, PROOF_VERIFIED), (2, PROOF_VERIFIED)]);
        let s = Spec { log: shown, pcr: vec![(14, extended.pcrs[14])], ..Spec::default() };
        assert_eq!(refused(s), Refusal::Replay(14), "kind {kind}");
    }
}
