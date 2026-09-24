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

//! Naming which part of the TLS handshake gave up.

use nonos_tls::SessionError;

use crate::trace;

use super::session::LinkError;

/*
 * Traced before it is flattened. Every guard on one boot failed here and "TLS
 * failed" was all eleven attempts had to say.
 */
pub(super) fn tls_fault(cause: SessionError) -> LinkError {
    // An alert is the relay's own account of what it objected to, so it is said
    // with the name the standard gives it and the number beside it.
    if let SessionError::PeerAlert(description) = cause {
        trace::say(nonos_tls::alert_name(description).as_bytes());
        trace::say_num(b"guard tls: peer alert", description as u64);
        return LinkError::Tls;
    }
    trace::say(match cause {
        SessionError::Init => b"guard tls: client hello could not be built" as &[u8],
        SessionError::Io => b"guard tls: socket failed mid handshake",
        SessionError::Handshake => b"guard tls: server flight never completed",
        SessionError::Certificate => b"guard tls: chain did not verify",
        SessionError::TooLarge => b"guard tls: server sent more than allowed",
        // Handled above, with its description.
        SessionError::PeerAlert(_) => b"guard tls: peer alert",
        SessionError::RetryUnsupported => b"guard tls: wants another key exchange group",
    });
    LinkError::Tls
}
