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

//! An enrolment certificate, `nsh-enrol/1`, as plain fields, signed by
//! whichever key a proof names, with either half of the signature
//! spoilt on demand.

use super::keys::{self, Hybrid};
pub use super::sig::{signed, SigMode};
use super::{put8, AK, FLEET, HOST, NOT_AFTER, PCRS};
use nsh_crypto::{labelled, Label};

#[derive(Clone)]
pub struct EnrolSpec {
    pub fleet: [u8; 16],
    pub host: Vec<u8>,
    pub ak: [u8; 64],
    pub host_keys: [u8; 64],
    pub pcrs: [[u8; 32]; 4],
    pub not_after: u64,
    pub authority_id: [u8; 32],
    pub signer: &'static Hybrid,
    pub sig: SigMode,
}

impl EnrolSpec {
    pub fn good() -> Self {
        let a = keys::authority();
        EnrolSpec {
            fleet: FLEET,
            host: HOST.to_vec(),
            ak: AK,
            host_keys: keys::host().digest(),
            pcrs: PCRS,
            not_after: NOT_AFTER,
            authority_id: a.id(),
            signer: a,
            sig: SigMode::Good,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = b"NSHE".to_vec();
        out.extend_from_slice(&[1, 0, 0, 0]);
        out.extend_from_slice(&self.fleet);
        put8(&mut out, &self.host);
        out.extend_from_slice(&self.ak);
        out.extend_from_slice(&self.host_keys);
        self.pcrs.iter().for_each(|p| out.extend_from_slice(p));
        out.extend_from_slice(&[1, 1]);
        out.extend_from_slice(&[0xB0; 32]);
        out.extend_from_slice(&self.not_after.to_le_bytes());
        out.extend_from_slice(&self.authority_id);
        let sig = signed(self.signer, &labelled(Label::Enrol, &out), self.sig);
        out.extend_from_slice(&sig);
        out
    }
}
