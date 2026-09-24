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

//! Driving the flight to Finished and deriving the application keys.

use crate::flight::ClientFlight;
use crate::handshake_records::handshake_messages;

use super::types::ServerComplete;

pub(super) fn complete(
    client: &ClientFlight,
    bytes: &[u8],
    host: &[u8],
    now: u64,
    require_chain: bool,
) -> Option<ServerComplete> {
    let mut ctx = crate::server_keys::server_keys(client, bytes)?;
    let msgs = handshake_messages(&ctx.keys, ctx.used, bytes)?;
    let mut scan = crate::scan_server_finished::ScanState {
        secret: &ctx.keys.server_secret,
        transcript: &mut ctx.transcript,
        host,
        now,
        cert11: &mut ctx.cert11,
        validated: &mut ctx.validated,
        require_chain,
    };
    if !crate::scan_server_finished::scan(&msgs, &mut scan) {
        return None;
    }
    let app = crate::app_keys::app_keys(&ctx.keys, &ctx.transcript)?;
    Some(ServerComplete {
        handshake: ctx.keys,
        app,
        transcript: ctx.transcript,
        certificates: ctx.cert11,
    })
}
