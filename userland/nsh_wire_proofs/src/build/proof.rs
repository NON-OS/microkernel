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

//! `HOST_PROOF` and `USER_AUTH` bodies, built from an enrolment and a
//! grant builder.

use super::enrol::EnrolB;
use super::grant::GrantB;
use super::{public, put16, put32, sig};

pub struct HostProofB {
    pub enrol: Vec<u8>,
    pub doc: Vec<u8>,
    pub entries: Vec<u8>,
}

impl Default for HostProofB {
    fn default() -> Self {
        HostProofB { enrol: EnrolB::default().bytes(), doc: vec![0xD0; 900], entries: vec![7; 90] }
    }
}

impl HostProofB {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        public(&mut out, 0x4B);
        put16(&mut out, &self.enrol);
        put32(&mut out, &self.doc);
        put32(&mut out, &self.entries);
        sig(&mut out, 0x5C);
        out
    }
}

/// A `USER_AUTH` body around `grant`.
pub fn user_auth(grant: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    public(&mut out, 0x55);
    put16(&mut out, grant);
    sig(&mut out, 0x5A);
    out
}

/// An `ESCALATE` body around `grant`.
pub fn escalate(grant: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    put16(&mut out, grant);
    out
}

/// The default grant, as a `USER_AUTH`.
pub fn default_user_auth() -> Vec<u8> {
    user_auth(&GrantB::default().bytes())
}
