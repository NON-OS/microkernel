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

//! The host that passes.

use nsh_attest::steps::NETWORK;

use super::keys::HOST;
use super::parts::{entry, entry_by, release};
use super::sig::tpmt;
use super::spec::Spec;
use super::tcg_event::digest;
use super::tcg_log::Log;

impl Default for Spec {
    fn default() -> Self {
        Spec {
            version: 5,
            th: [0x77; 64],
            host: b"db-1.eu.example".to_vec(),
            now_ms: 5_000,
            log: Log::standard().with_capsules(),
            pcr: Vec::new(),
            quoted: Vec::new(),
            challenge: None,
            dma: (1, true, 0, 0),
            bound_dma: None,
            quote: |_| {},
            after_sign: |_| {},
            sig: |r, s| tpmt(0x0018, 0x000B, &r, &s),
            signing_key: 1,
            doc_key: 1,
            doc: |_| {},
            bytes: |_| {},
            entries: vec![
                entry_by(0, [0xFA; 32], 0, 0),
                entry_by(5, [0xF0; 32], NETWORK | 1 << 3, 0),
                entry(6, [0xA7; 32], 1 << 9),
            ],
            root: None,
            cert: |_| {},
            cert_signer: 1,
            cert_bytes: |_| {},
            releases: vec![release()],
            revoke: false,
            boot_apps: vec![digest(b"boot manager")],
            host_key: HOST,
        }
    }
}
