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

//! The document: registry, challenge, binding, quote, signature, layout.

use nonos_hash::sha256;
use nsh_attest::consts::QUOTED_PCRS_V4;
use nsh_attest::steps::binding::{qualifying_data, DmaPosture};
use nsh_attest::steps::fold;
use nsh_crypto::attest_challenge;

use super::ak::Key;
use super::attest::Quote;
use super::doc::Parts;
use super::keys::{hybrid, HOST};
use super::Spec;

impl Spec {
    pub fn doc_bytes(&self) -> Vec<u8> {
        let root = self.root.unwrap_or_else(|| fold(&self.entries.concat()));
        let host = hybrid(HOST);
        let ch = attest_challenge(&self.th, &host.ed.public, &host.ml_public[..]);
        let challenge = self.challenge.unwrap_or(ch);
        let (v, e, g, i) = self.bound_dma.unwrap_or(self.dma);
        let posture =
            DmaPosture { vendor: v, enforcing: e, unconfined_grants: g, identity_devices: i };
        let extra = qualifying_data(&challenge, &root, &posture);
        let mut quote = Quote::honest(&extra, &QUOTED_PCRS_V4, &self.quoted_values());
        (self.quote)(&mut quote);
        let mut attest = quote.bytes();
        let (r, s) = Key::new(self.signing_key).sign(&sha256(&attest), 0);
        (self.after_sign)(&mut attest);
        let (vendor, enforcing, grants, identity) = self.dma;
        let mut parts = Parts {
            version: self.version,
            challenge,
            root,
            count: self.entries.len() as u32,
            flags: [1, vendor, enforcing as u8],
            grants,
            identity,
            pcrs: QUOTED_PCRS_V4.iter().copied().zip(self.values()).collect(),
            log: self.log.bytes.clone(),
            attest,
            sig: (self.sig)(r, s),
            ak: Key::new(self.doc_key).public().to_vec(),
        };
        (self.doc)(&mut parts);
        let mut doc = if self.version <= 4 { parts.kernel(self.version) } else { parts.v5() };
        (self.bytes)(&mut doc);
        doc
    }
}
