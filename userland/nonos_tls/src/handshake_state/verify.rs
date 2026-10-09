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

//! Verifying a complete flight, once.

use alloc::vec::Vec;

use super::types::HandshakeState;
use crate::scan_server_finished::{scan, ScanState};
use crate::server_complete::ServerComplete;

impl HandshakeState {
    /*
     * Nothing is skipped: the chain when `require_chain`, the CertificateVerify
     * signature and the Finished MAC are all checked, in that order, over the
     * messages decrypted while the flight arrived. What is gone is doing it
     * twice, and decrypting the flight again to do it.
     */
    /// Check the finished flight and derive the application keys. `None` if
    /// the flight is not complete or any check fails.
    pub fn verify(&self, host: &[u8], now: u64, require_chain: bool) -> Option<ServerComplete> {
        self.end?;
        let mut transcript = self.transcript.clone();
        let mut certificates = Vec::new();
        let mut validated = false;
        let mut request_context = None;
        let mut state = ScanState {
            secret: &self.keys.server_secret,
            transcript: &mut transcript,
            host,
            now,
            cert11: &mut certificates,
            validated: &mut validated,
            require_chain,
            request_context: &mut request_context,
        };
        if !scan(&self.msgs, &mut state) {
            return None;
        }
        let transcript_hash = transcript.digest();
        let app = crate::app_keys::app_keys(&self.keys, &transcript_hash)?;
        let client_certificate = request_context.map(|context| empty_certificate(&context));
        let finished_hash = match &client_certificate {
            Some(message) => {
                transcript.push(message);
                transcript.digest()
            }
            None => transcript_hash,
        };
        Some(ServerComplete {
            handshake: self.keys,
            app,
            transcript_hash,
            certificates,
            client_certificate,
            finished_hash,
        })
    }
}

/// A Certificate message that echoes `context` and carries no certificate,
/// which is how a client declines a CertificateRequest (RFC 8446 4.4.2).
fn empty_certificate(context: &[u8]) -> Vec<u8> {
    let body_len = 1 + context.len() + 3;
    let mut message = Vec::with_capacity(4 + body_len);
    message.push(11);
    crate::push::u24(&mut message, body_len);
    message.push(context.len() as u8);
    message.extend_from_slice(context);
    crate::push::u24(&mut message, 0);
    message
}
