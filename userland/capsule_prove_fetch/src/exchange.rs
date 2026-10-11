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

//! One connection to the relay: TLS with the chain checked, the request,
//! then the answer, each read within the route's budget and the op's end.
//! `Err(None)` is worth another connection; `Err(Some)` ends the op.

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;
use nonos_proof_abi::wire::fetch::RELAY;
use nonos_tls::stream::connect;
use nonos_tls::{rtc_now, SessionError};

use crate::desk::{verdict, Read, Refused, Reply};
use crate::io::RouteIo;

pub fn exchange(
    io: &mut RouteIo,
    host: &str,
    req: &[u8],
    get: bool,
    (read_ms, until): (i64, i64),
) -> Result<Vec<u8>, Option<i32>> {
    let mut tls = connect(io, host.as_bytes(), rtc_now()).map_err(|e| match e {
        SessionError::Certificate => Some(RELAY),
        _ => None,
    })?;
    tls.write_all(io, req).map_err(|_| None)?;
    let (mut reply, mut last) = (Reply::new(get), mk_uptime_ms());
    loop {
        let came = tls.read(io).map_err(|_| None)?;
        let now = mk_uptime_ms();
        if came.is_empty() {
            if tls.is_done() || now.saturating_sub(last) > read_ms || now >= until {
                return Err(None);
            }
            continue;
        }
        last = now;
        if let Read::Done(code, b) = reply.feed(&came).map_err(|Refused| Some(RELAY))? {
            return verdict(!get, code, b);
        }
    }
}
