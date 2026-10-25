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

//! `nsh-enrol/1` certificates, laid out as nsh-v1-attest.md gives them
//! (little-endian, as every nsh/1 body is) and signed by an authority.

use nsh_crypto::{labelled, Label};

use super::hybrid::Hybrid;

#[derive(Clone)]
pub struct Cert {
    pub fleet: [u8; 16],
    pub host: Vec<u8>,
    pub ak: [u8; 64],
    pub host_keys: [u8; 64],
    pub pcrs: [[u8; 32]; 4],
    pub boot_apps: Vec<[u8; 32]>,
    pub not_after_ms: u64,
    pub authority_id: [u8; 32],
}

impl Cert {
    pub fn signed_part(&self) -> Vec<u8> {
        let mut b = b"NSHE".to_vec();
        b.extend(1u16.to_le_bytes());
        b.extend(0u16.to_le_bytes());
        b.extend(self.fleet);
        b.push(self.host.len() as u8);
        b.extend(&self.host);
        b.extend(self.ak);
        b.extend(self.host_keys);
        self.pcrs.iter().for_each(|p| b.extend(p));
        b.extend([1, self.boot_apps.len() as u8]);
        self.boot_apps.iter().for_each(|a| b.extend(a));
        b.extend(self.not_after_ms.to_le_bytes());
        b.extend(self.authority_id);
        b
    }

    /// The certificate, signed by `by`.
    pub fn sign(&self, by: &Hybrid) -> Vec<u8> {
        let mut b = self.signed_part();
        b.extend(by.sign(&labelled(Label::Enrol, &b)));
        b
    }
}
