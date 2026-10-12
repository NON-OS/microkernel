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

//! The responder's phase 1 and public key exchange, its identity keys,
//! and the number its user would see.

use super::peer::Peer;
use nonos_bt_smp::ecdh::PeerKey;
use nonos_bt_smp::funcs::{f4, g2, passkey_display};
use nonos_bt_smp::method::select;
use nonos_bt_smp::pdu::*;

impl Peer {
    pub(super) fn on_request(&mut self, p: &[u8]) -> Vec<Vec<u8>> {
        let req = Pairing::parse(p).unwrap();
        let mitm = |p: &Pairing| p.auth_req & auth::MITM != 0;
        self.method = select(req.io_cap, mitm(&req), self.ours.io_cap, mitm(&self.ours));
        self.req = Some(req);
        vec![frame(PAIRING_RESPONSE, &self.ours.params()).unwrap()]
    }

    pub(super) fn on_public_key(&mut self, p: &[u8]) -> Vec<Vec<u8>> {
        let x: [u8; 32] = p[..32].try_into().unwrap();
        let y: [u8; 32] = p[32..].try_into().unwrap();
        self.pka = be(&x);
        let ours = match (self.faults.reflect, self.faults.key) {
            (true, _) => p.to_vec(),
            (_, Some(k)) => k.to_vec(),
            _ => [le(self.keys.public().0), le(self.keys.public().1)].concat(),
        };
        let mut out = vec![frame(PAIRING_PUBLIC_KEY, &ours).unwrap()];
        if let Ok(k) = PeerKey::new(&self.pka, &be(&y)) {
            self.dh = self.keys.dh_key(&k).unwrap();
        }
        if self.z() == 0 {
            out.push(self.commit());
        }
        out
    }

    /// A fresh Nb and its confirm Cb = f4(PKbx, PKax, Nb, rb).
    pub(super) fn commit(&mut self) -> Vec<u8> {
        nonos_bt_smp::pairing::Random::fill(&mut self.rng, &mut self.nb);
        let mut cb = f4(self.keys.public().0, &self.pka, &self.nb, self.z());
        cb[0] ^= self.faults.bad_confirm as u8;
        frame(PAIRING_CONFIRM, &le(&cb)).unwrap()
    }

    pub fn keys(&self) -> Vec<Vec<u8>> {
        let irk = frame(IDENTITY_INFORMATION, &le(&self.irk)).unwrap();
        let addr = frame(IDENTITY_ADDRESS_INFORMATION, &self.identity).unwrap();
        if self.faults.keys_reversed {
            vec![addr, irk]
        } else {
            vec![irk, addr]
        }
    }

    pub fn number(&self) -> u32 {
        passkey_display(g2(&self.pka, self.keys.public().0, &self.na, &self.nb))
    }
}
