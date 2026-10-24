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

//! The enrolment's expiry, its withdrawal, and a policy with no
//! authority: each differs from the good run only there.

use super::host_run;
use crate::kit::attest::TestAttester;
use crate::kit::enrol::EnrolSpec;
use crate::kit::pair::config;
use crate::kit::proof::ProofSpec;
use nonos_hash::sha512;
use nsh_client::{EnrolRefusal, HostRefusal, Refusal};

fn enrol(e: EnrolRefusal) -> Refusal {
    Refusal::Host(HostRefusal::Enrol(e))
}

#[test]
fn a_withdrawn_certificate_is_refused() {
    let mut cfg = config();
    cfg.policy.revoked = vec![[0; 64], sha512(&EnrolSpec::good().bytes())];
    let (r, _) = host_run(&ProofSpec::good(), cfg, TestAttester::default());
    assert_eq!(r, enrol(EnrolRefusal::Revoked));
}

#[test]
fn an_empty_policy_trusts_no_enrolment() {
    let mut cfg = config();
    cfg.policy.authorities.clear();
    let (r, _) = host_run(&ProofSpec::good(), cfg, TestAttester::default());
    assert_eq!(r, enrol(EnrolRefusal::Authority));
}
