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

//! A spec, built: the bytes a host sends and the policy a client holds.
//! `HOST_PROOF` is signed afresh by `host_key` on every check, so a test
//! may change any part after building.

use nonos_hash::sha512;
use nsh_attest::{Evidence, Policy, Refusal, Release, Session};
use nsh_crypto::Transcript;

use super::keys::{hybrid, AUTHORITY};
use super::proof::host_proof;

#[derive(Clone)]
pub struct Built {
    pub doc: Vec<u8>,
    pub enrol: Vec<u8>,
    pub entries: Vec<u8>,
    pub th: [u8; 64],
    pub host: Vec<u8>,
    pub now_ms: u64,
    pub fleet: [u8; 16],
    pub releases: Vec<Release>,
    pub revoked: Vec<[u8; 64]>,
    pub host_key: u8,
}

impl Built {
    /// `HOST_PROOF` as the host sends it, signed over a fresh transcript.
    pub fn proof(&self) -> Vec<u8> {
        host_proof(self, &Transcript::default())
    }

    pub fn session(&self) -> Session<'_> {
        Session { th_server_hello: &self.th, host: &self.host, now_ms: self.now_ms }
    }

    pub fn policy(&self) -> Policy {
        let auth = vec![hybrid(AUTHORITY).authority()];
        Policy::new(self.fleet, auth, self.releases.clone(), self.revoked.clone()).expect("policy")
    }

    /// The whole check: the release's version and PCR 9, or the refusal.
    pub fn check(&self) -> Result<(String, [u8; 32]), Refusal> {
        let (body, policy) = (self.proof(), self.policy());
        let ev = Evidence::verify(&body, &Transcript::default()).expect("signed by the host");
        let v = nsh_attest::check(&ev, &self.session(), &policy)?;
        Ok((v.release.version.clone(), v.pcr9))
    }

    pub fn revoke_own(&mut self) {
        self.revoked.push(sha512(&self.enrol));
    }
}
