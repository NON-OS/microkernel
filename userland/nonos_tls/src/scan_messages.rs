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

// Handshake message types, RFC 8446 section 4.
pub(super) const CERTIFICATE: u8 = 11;
pub(super) const CERTIFICATE_VERIFY: u8 = 15;
pub(super) const FINISHED: u8 = 20;

/// Certificate. Keeps the chain and walks it, unless the caller has taken
pub(super) fn certificate(body: &[u8], state: &mut ScanState) -> bool {
    state.cert11.clear();
    state.cert11.extend_from_slice(body);
    if state.require_chain
        && !super::chain_walk::verify_chain(state.cert11.as_slice(), state.host, state.now)
    {
        return false;
    }
    true
}

pub(super) fn certificate_verify(body: &[u8], state: &mut ScanState) -> bool {
    let before = state.transcript.clone();
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
    super::finished_verify::verify(state.secret, state.transcript, body)
}
