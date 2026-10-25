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

//! The certificate, and everything together.

use nsh_crypto::key_digest;

use super::ak::Key;
use super::enrol::Cert;
use super::keys::{hybrid, AUTHORITY, HOST};
use super::{Built, Spec};

impl Spec {
    pub fn cert_bytes(&self) -> Vec<u8> {
        let (host, l) = (hybrid(HOST), &self.log.pcrs);
        let mut cert = Cert {
            fleet: [0xF1; 16],
            host: b"db-1.eu.example".to_vec(),
            ak: Key::new(1).public(),
            host_keys: key_digest(&host.ed.public, &host.ml_public[..]),
            pcrs: [l[0], l[1], l[2], l[7]],
            boot_apps: self.boot_apps.clone(),
            not_after_ms: 10_000,
            authority_id: *hybrid(AUTHORITY).authority().id(),
        };
        (self.cert)(&mut cert);
        let mut enrol = cert.sign(hybrid(self.cert_signer));
        (self.cert_bytes)(&mut enrol);
        enrol
    }

    pub fn build(&self) -> Built {
        let mut built = Built {
            doc: self.doc_bytes(),
            enrol: self.cert_bytes(),
            entries: self.entries.concat(),
            th: self.th,
            host: self.host.clone(),
            now_ms: self.now_ms,
            fleet: [0xF1; 16],
            releases: self.releases.clone(),
            revoked: Vec::new(),
            host_key: self.host_key,
        };
        if self.revoke {
            built.revoke_own();
        }
        built
    }
}
