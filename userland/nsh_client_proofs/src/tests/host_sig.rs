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

//! The host keys' hybrid signature over the transcript. Both halves must
//! verify: a run whose `HOST_PROOF` differs only in one half is refused,
//! naming that half. So are keys other than the signer's, and a body
//! changed after it was signed.

use super::host_refusal;
use crate::kit::attest::TestAttester;
use crate::kit::enrol::SigMode;
use crate::kit::keys;
use crate::kit::pair::{config, Pair};
use crate::kit::proof::ProofSpec;
use nsh_client::{Half, HostRefusal, Refusal};
use nsh_wire::{Kind, Reason};

fn sig(h: Half) -> Refusal {
    Refusal::Host(HostRefusal::HostSig(h))
}

#[test]
fn each_half_of_the_host_signature_is_checked() {
    let spec = ProofSpec { sig: SigMode::BadEd, ..ProofSpec::good() };
    assert_eq!(host_refusal(&spec), sig(Half::Ed25519));
    let spec = ProofSpec { sig: SigMode::BadMl, ..ProofSpec::good() };
    assert_eq!(host_refusal(&spec), sig(Half::MlDsa65));
}

#[test]
fn a_signature_by_other_keys_is_refused() {
    let spec = ProofSpec { signer: keys::stranger(), ..ProofSpec::good() };
    assert_eq!(host_refusal(&spec), sig(Half::Ed25519));
    let spec = ProofSpec { host: keys::stranger(), ..ProofSpec::good() };
    assert_eq!(host_refusal(&spec), sig(Half::Ed25519));
}

#[test]
fn the_signature_covers_the_transcript_not_only_the_body() {
    let mut p = Pair::with(config(), TestAttester::default());
    p.to_proof();
    let body = ProofSpec::good().bytes(&p.server);
    p.server.th.push(Kind::Escalate, b"a frame the client never saw");
    let body_again = ProofSpec::good().bytes(&p.server);
    let mut spliced = body.clone();
    let sig_at = body.len() - 3373;
    spliced[sig_at..].copy_from_slice(&body_again[sig_at..]);
    let f = p.server.seal(Kind::HostProof, &spliced);
    assert_eq!(p.feed(&f), Err(sig(Half::Ed25519)));
    super::aborted_with(&mut p, Reason::Policy);
}
