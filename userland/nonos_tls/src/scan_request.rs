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

//! The CertificateRequest a server may send before its own Certificate.

use super::scan_state::ScanState;

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
