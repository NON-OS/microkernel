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

//! TLS 1.3 walked as nonos_tls's blocking session walks it, a step at a
//! time: the hello, the server's flight until a whole Finished, then the
//! chain, CertificateVerify and Finished checked once, and the request
//! sealed only if all of them passed. The response's records are opened as
//! they complete, each once.

use alloc::vec::Vec;

use nonos_tls::{client_flight, AppReader, Refusal};

use super::types::{After, Exchange, Phase, Tls};

#[path = "tls_check.rs"]
mod check;

const NO_HELLO: &str = "tls: the hello could not be built";
const FAILED: &str = "tls handshake failed";
const REFUSED: &str = "the server ended the handshake with an alert";
const RETRY: &str = "the server asked for a key exchange this client does not offer";
const UNVERIFIED: &str = "the certificate did not verify for this host";

/// What a refusal of the server's flight says to a person.
fn refused(r: Refusal) -> &'static str {
    match r {
        Refusal::Alert(_) => REFUSED,
        Refusal::Unverified => UNVERIFIED,
        Refusal::Incomplete | Refusal::Seal => FAILED,
    }
}

impl Exchange {
    pub(super) fn hello(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let client = client_flight(self.host.as_bytes()).ok_or(NO_HELLO)?;
        self.out = client.record.clone();
        self.sent = 0;
        self.raw = Vec::new();
        self.tls = Some(Tls { client, handshake: None, app: None, reader: AppReader::new() });
        self.phase = Phase::Send(After::Flight);
        Ok(None)
    }

    /// Open the response records completed since the last step.
    pub(super) fn open_records(&mut self) {
        if let Some(Tls { app: Some(app), reader, .. }) = self.tls.as_mut() {
            reader.feed(app, &self.raw);
        }
    }
}
