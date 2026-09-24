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

use crate::browser::fetch::tls::flight_settled;
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::{append_capped, constants};
use crate::browser::net;
use crate::browser::tls13;

pub(in crate::browser::fetch) fn read_flight(port: u32, f: &mut Fetch) {
    let mut chunk = [0u8; 4096];
    let Some(tls) = f.tls.as_mut() else {
        f.phase = Phase::Error;
        return;
    };
    let mut got = false;
    for _ in 0..constants::DRAIN_BURST {
        match net::socket_recv(port, f.handle, &mut chunk) {
            Ok(n) if n > 0 => {
                got = true;
                if append_capped::append_capped(
                    &mut tls.flight,
                    &chunk[..n],
                    constants::MAX_TLS_FLIGHT,
                )
                .is_err()
                {
                    f.error = Some("tls flight too large");
                    f.phase = Phase::Error;
                    return;
                }
                if tls13::server_finished_flight_ready(&tls.flight) {
                    super::trace::flight(b"complete", tls.flight.len(), f.idle);
                    f.phase = Phase::TlsVerify;
                    return;
                }
                /*
                 * A server that refuses the hello answers with an alert in the
                 * clear, and no ServerHello is ever coming. Without this the
                 * loop drains, the flight never reads as ready, and a refusal
                 * we were told about in the first packet is reported as a
                 * handshake that timed out.
                 */
                if let Some(description) = tls13::description_in_record(&tls.flight) {
                    super::trace::flight(b"refused", tls.flight.len(), f.idle);
                    f.tls_alert = Some(description);
                    f.error = Some("tls handshake refused");
                    f.phase = Phase::Error;
                    return;
                }
            }
            _ => break,
        }
    }
    if got {
        f.idle = 0;
    } else {
        let (settled, have) = (flight_settled(&tls.flight), tls.flight.len());
        super::flight_quiet::quiet(f, settled, have);
    }
}
