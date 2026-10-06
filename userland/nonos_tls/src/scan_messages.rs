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

//! What the scan does with each handshake message it recognises.

use super::scan_server_finished::ScanState;

/* Handshake message types, RFC 8446 section 4. */
pub(super) const CERTIFICATE: u8 = 11;
pub(super) const CERTIFICATE_REQUEST: u8 = 13;
pub(super) const CERTIFICATE_VERIFY: u8 = 15;
pub(super) const FINISHED: u8 = 20;

/// Certificate. Keeps the chain and walks it, unless the caller has taken
///
/// The server sends one, before the CertificateVerify that signs with its key
/// (RFC 8446 section 4.4). A second one would replace the kept chain after the
/// signature had been checked against the first, and a caller that pins the
/// leaf itself would then be handed a key that signed nothing.
/*
 * A Tor-lineage relay asks every peer for a certificate (it sets
 * SSL_VERIFY_PEER and accepts whatever comes), so OpenSSL sends this between
 * EncryptedExtensions and the server's Certificate. Only its context is kept:
 * the client offers no certificate, so the extensions it lists are not read.
 * It may come once, and only before the server's own Certificate.
 */
pub(super) fn certificate_request(body: &[u8], state: &mut ScanState) -> bool {
    if state.request_context.is_some() || !state.cert11.is_empty() || *state.validated {
        return false;
    }
    let Some(&context_len) = body.first() else {
        return false;
    };
    let context_end = 1 + context_len as usize;
    let Some(context) = body.get(1..context_end) else {
        return false;
    };
    let Some(ext_len) = super::read::u16_at(body, context_end) else {
        return false;
    };
    if context_end + 2 + ext_len as usize != body.len() {
        return false;
    }
    *state.request_context = Some(context.to_vec());
    true
}

pub(super) fn certificate(body: &[u8], state: &mut ScanState) -> bool {
    if *state.validated || !state.cert11.is_empty() {
        return false;
    }
    state.cert11.extend_from_slice(body);
    if state.require_chain
        && !super::chain_walk::verify_chain(state.cert11.as_slice(), state.host, state.now)
    {
        return false;
    }
    true
}

/*
 * The signature covers the transcript hash up to, not including, this
 * message. The running hash answers that without copying the transcript.
 */
pub(super) fn certificate_verify(body: &[u8], state: &mut ScanState) -> bool {
    if *state.validated {
        return false;
    }
    let before = state.transcript.digest();
    let Some(leaf) = super::cert_at::cert_at(state.cert11.as_slice(), 0) else {
        return false;
    };
    if !super::cert_verify_msg::verify_cert_verify(leaf, &before, body) {
        return false;
    }
    *state.validated = true;
    true
}

pub(super) fn finished(body: &[u8], state: &mut ScanState) -> bool {
    if !*state.validated {
        return false;
    }
    super::finished_verify::verify(state.secret, &state.transcript.digest(), body)
}
