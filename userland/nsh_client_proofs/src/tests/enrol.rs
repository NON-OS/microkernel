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

//! The enrolment certificate. Each run differs from the good one in one
//! field of the certificate, or in the policy the client holds, and is
//! refused for exactly that.

use super::{host_refusal, host_run};
use crate::kit::attest::TestAttester;
use crate::kit::enrol::{EnrolSpec, SigMode};
use crate::kit::keys;
use crate::kit::pair::{config, Pair};
use crate::kit::proof::ProofSpec;
use crate::kit::NOT_AFTER;
use nsh_client::{EnrolRefusal, Half, HostRefusal, Refusal};

fn with(edit: impl FnOnce(&mut EnrolSpec)) -> Refusal {
    let mut spec = ProofSpec::good();
    edit(&mut spec.enrol);
    host_refusal(&spec)
}

fn enrol(e: EnrolRefusal) -> Refusal {
    Refusal::Host(HostRefusal::Enrol(e))
}

#[test]
fn the_certificate_must_be_the_fleet_authoritys_both_halves() {
    assert_eq!(with(|e| e.authority_id = keys::stranger().id()), enrol(EnrolRefusal::Authority));
    assert_eq!(with(|e| e.signer = keys::stranger()), enrol(EnrolRefusal::Sig(Half::Ed25519)));
    assert_eq!(with(|e| e.sig = SigMode::BadEd), enrol(EnrolRefusal::Sig(Half::Ed25519)));
    assert_eq!(with(|e| e.sig = SigMode::BadMl), enrol(EnrolRefusal::Sig(Half::MlDsa65)));
}

#[test]
fn the_certificate_must_name_this_fleet_ak_host_keys_and_host() {
    assert_eq!(with(|e| e.fleet = [0xF2; 16]), enrol(EnrolRefusal::Fleet));
    assert_eq!(with(|e| e.ak[63] ^= 1), enrol(EnrolRefusal::Ak));
    assert_eq!(with(|e| e.host_keys = keys::stranger().digest()), enrol(EnrolRefusal::HostKeys));
    assert_eq!(with(|e| e.host = b"db-2.eu.example".to_vec()), enrol(EnrolRefusal::HostName));
    assert_eq!(with(|e| e.host = b"db-1.eu.example.evil".to_vec()), enrol(EnrolRefusal::HostName));
}

#[test]
fn the_certificate_must_not_have_expired() {
    let at = |now| {
        let cfg = nsh_client::Config { now_ms: now, ..config() };
        host_run(&ProofSpec::good(), cfg, TestAttester::default()).0
    };
    assert_eq!(at(NOT_AFTER), enrol(EnrolRefusal::Expired));
    assert_eq!(at(NOT_AFTER + 1), enrol(EnrolRefusal::Expired));
    let mut p = Pair::with(
        nsh_client::Config { now_ms: NOT_AFTER - 1, ..config() },
        TestAttester::default(),
    );
    p.to_proof();
    let hp = p.proof(&ProofSpec::good());
    assert_eq!(p.feed(&hp), Ok(()), "valid until its last instant");
}
