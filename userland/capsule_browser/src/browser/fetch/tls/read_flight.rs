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

use super::flight_state::{judge, Flight};
use crate::browser::fetch::constants::MAX_TLS_FLIGHT;
use crate::browser::fetch::deadline::read_ms;
use crate::browser::fetch::types::Fetch;
use crate::browser::fetch::wire::Wire;
use crate::browser::net::drain::drain;

/// Take what the server has sent of its flight, and act in this call on
/// what it amounts to: answer a finished flight, stop on a refused one.
pub(in crate::browser::fetch) fn read_flight<W: Wire>(w: &mut W, f: &mut Fetch, until: i64) {
    let Some(tls) = f.tls.as_mut() else { return f.stop("tls handshake failed") };
    let budget = read_ms(w.now_ms(), until);
    let read = drain(w, f.handle, &mut tls.flight, MAX_TLS_FLIGHT, budget);
    let verdict = judge(tls);
    let have = tls.flight.len();
    if read.got > 0 {
        f.progress_ms = w.now_ms();
        f.received += read.got;
    }
    match verdict {
        Flight::Waiting if read.full => f.stop("tls flight too large"),
        Flight::Waiting => {}
        Flight::Complete(end) => {
            super::trace::flight(w, "complete", have);
            super::verify_and_send::verify_and_send(w, f, end);
        }
        Flight::Refused(alert) => {
            super::trace::flight(w, "refused", have);
            f.tls_alert = alert;
            f.stop("tls handshake refused");
        }
        Flight::Failed => {
            super::trace::flight(w, "failed", have);
            f.stop("tls handshake failed");
        }
    }
}
