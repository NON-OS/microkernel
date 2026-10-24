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

//! The attestation check. Its refusal, whichever step it names, ends the
//! session with `ABORT(POLICY)` and no credential. A document for another
//! handshake is refused by the attester, and by the client itself when an
//! attester is lax about it. The attester is asked only after the host's
//! own signature has passed.

use super::{host_refusal, host_run};
use crate::kit::attest::{doc, TestAttester};
use crate::kit::enrol::SigMode;
use crate::kit::pair::config;
use crate::kit::proof::ProofSpec;
use nsh_client::{AttestRefusal, HostRefusal, Refusal};

fn refused(r: AttestRefusal) {
    let attester = TestAttester { refuse: Some(r), ..TestAttester::default() };
    let (got, p) = host_run(&ProofSpec::good(), config(), attester);
    assert_eq!(got, Refusal::Host(HostRefusal::Attest(r)));
    assert_eq!(p.client.attester().calls.get(), 1);
}

#[test]
fn each_attestation_refusal_ends_the_session_before_any_credential() {
    use AttestRefusal::*;
    [Parse, Version, Challenge, Quote, ReleasePcrs, Dma, Registry, DeviceProof, Unavailable]
        .into_iter()
        .for_each(refused);
}

#[test]
fn a_document_for_another_handshake_is_refused() {
    let spec = ProofSpec { doc: Some(doc(&[0x99; 32])), ..ProofSpec::good() };
    assert_eq!(host_refusal(&spec), Refusal::Host(HostRefusal::Attest(AttestRefusal::Challenge)));
    let lax = TestAttester { lax: true, ..TestAttester::default() };
    let (got, p) = host_run(&spec, config(), lax);
    assert_eq!(got, Refusal::Host(HostRefusal::Challenge), "the client checks it again");
    assert!(p.client.attester().said_ok.get());
}

#[test]
fn a_document_that_is_not_one_is_refused() {
    let spec = ProofSpec { doc: Some(vec![0xD0; 300]), ..ProofSpec::good() };
    assert_eq!(host_refusal(&spec), Refusal::Host(HostRefusal::Attest(AttestRefusal::Parse)));
}

#[test]
fn the_attester_is_not_asked_until_the_host_signature_passes() {
    let spec = ProofSpec { sig: SigMode::BadMl, ..ProofSpec::good() };
    let (_, p) = host_run(&spec, config(), TestAttester::default());
    assert_eq!(p.client.attester().calls.get(), 0);
}
