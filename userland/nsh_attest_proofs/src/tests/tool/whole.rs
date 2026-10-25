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

//! The client's whole check of a host with the tool's certificate, under a
//! policy that holds the tool's authority.

use nsh_attest::{Authority, Evidence, Policy, Refusal};
use nsh_crypto::Transcript;

use super::key;
use crate::build::Built;

/// The client's whole check of `b` with `cert`, under the tool's authority.
pub fn check(b: &Built, cert: Vec<u8>) -> Result<(), Refusal> {
    let mut b = b.clone();
    b.enrol = cert;
    let public = key().public();
    let ml: [u8; 1952] = public[32..].try_into().expect("ML-DSA-65 key");
    let auth = Authority::new(public[..32].try_into().expect("Ed25519 key"), Box::new(ml));
    let policy = Policy::new(b.fleet, vec![auth], b.releases.clone(), vec![]).expect("policy");
    let proof = b.proof();
    let ev = Evidence::verify(&proof, &Transcript::default()).expect("signed by the host");
    nsh_attest::check(&ev, &b.session(), &policy).map(|_| ())
}
