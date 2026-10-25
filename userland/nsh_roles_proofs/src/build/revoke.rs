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

//! A valid `nsh-revoke/1` list: issued an hour before `NOW`, naming one
//! grant that no test logs in with, signed by the second authority.

use super::keys::{key, K};
use super::{put_u16, put_u64, Spoil, FLEET, HOUR, NOW};
use nsh_crypto::{labelled, Label};

#[derive(Debug, Clone)]
pub struct RevB {
    pub fleet: [u8; 16],
    pub issued_ms: u64,
    pub ids: Vec<[u8; 16]>,
    pub signer_id: [u8; 32],
    pub signer: K,
    pub spoil: Spoil,
}

impl Default for RevB {
    fn default() -> Self {
        RevB {
            fleet: FLEET,
            issued_ms: NOW - HOUR,
            ids: vec![[0x99; 16]],
            signer_id: key(K::AuthB).id(),
            signer: K::AuthB,
            spoil: Spoil::None,
        }
    }
}

impl RevB {
    pub fn signed(&self) -> Vec<u8> {
        let mut out = b"NSHV".to_vec();
        put_u16(&mut out, 1);
        put_u16(&mut out, 0);
        out.extend_from_slice(&self.fleet);
        put_u64(&mut out, self.issued_ms);
        put_u16(&mut out, u16::try_from(self.ids.len()).expect("too many ids"));
        self.ids.iter().for_each(|id| out.extend_from_slice(id));
        out.extend_from_slice(&self.signer_id);
        out
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.signed();
        let mut sig = key(self.signer).sign(&labelled(Label::Revoke, &out));
        self.spoil.apply(&mut sig);
        out.extend_from_slice(&sig);
        out
    }
}
