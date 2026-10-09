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

use crate::conn::Dest;
use crate::nym::{connect_request, open_session, SendError};

/// Why a tunnel did or did not open. These are kept apart all the way to the
/// SOCKS reply byte: they fail for unrelated reasons, and collapsing them into
/// one code leaves a client with nothing to report but "rejected" on a machine
/// whose only other channel is a serial port it does not have.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpenOutcome {
    Opened,
    /// The mixnet has no session, so nothing can be sent through it at all.
    NoSession,
    /// A session exists but no exit has been chosen to carry the destination.
    NoExit,
    /// The request was formed and refused on its way out.
    SendFailed,
    /// The request could not be built for this destination.
    BadRequest,
    /// net.nym no longer holds the session the request went out on.
    SessionLost,
}

impl OpenOutcome {
    /// The SOCKS reply code that says this to the client.
    pub fn reply_code(self) -> u8 {
        match self {
            OpenOutcome::Opened => crate::wire::REP_OK,
            OpenOutcome::NoSession => crate::wire::REP_NET_UNREACH,
            OpenOutcome::NoExit => crate::wire::REP_HOST_UNREACH,
            OpenOutcome::SendFailed | OpenOutcome::SessionLost => crate::wire::REP_CONN_REFUSED,
            OpenOutcome::BadRequest => crate::wire::REP_GENERAL_FAIL,
        }
    }

    /// A second session lost in a row is a send that failed.
    fn settled(self) -> Self {
        match self {
            OpenOutcome::SessionLost => OpenOutcome::SendFailed,
            outcome => outcome,
        }
    }
}

/// Ask the mixnet to open a tunnel to `dest`.
///
/// Returns whether the request left. It does not wait for the exit to answer:
/// a mixnet round trip is deliberately slow, and holding the SOCKS handshake
/// open across it would stall the client on a path built to add delay.
pub fn open_tunnel(conn_id: u64, dest: &Dest) -> OpenOutcome {
    super::trace::destination(dest);
    match try_open(conn_id, dest) {
        // net.nym had dropped the session this capsule held, and the failed
        // send has dropped it here too: one more try opens a fresh one,
        // rather than refusing this connection for a fault already mended.
        OpenOutcome::SessionLost => try_open(conn_id, dest).settled(),
        outcome => outcome,
    }
}

fn try_open(conn_id: u64, dest: &Dest) -> OpenOutcome {
    if open_session().is_none() {
        super::trace::open_failed(b"no session", 0);
        return OpenOutcome::NoSession;
    }
    match connect_request(conn_id, dest) {
        Ok(frame) => match crate::nym::send_through_mixnet(&frame) {
            Ok(()) => OpenOutcome::Opened,
            Err(SendError::Remote(crate::nym::E_NO_SESSION)) => {
                super::trace::open_failed(b"session gone, opening another", 0);
                OpenOutcome::SessionLost
            }
            Err(SendError::Remote(code)) => {
                super::trace::open_failed(b"send", code);
                OpenOutcome::SendFailed
            }
            Err(_) => {
                super::trace::open_failed(b"send", 0);
                OpenOutcome::SendFailed
            }
        },
        Err(SendError::NoExit) => {
            super::trace::open_failed(b"no exit", 0);
            OpenOutcome::NoExit
        }
        Err(_) => {
            super::trace::open_failed(b"request", 0);
            OpenOutcome::BadRequest
        }
    }
}
