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

use crate::browser::fetch::plain::request;
use crate::browser::fetch::types::{Fetch, Phase, TlsWhy};
use crate::browser::fetch::wire::Wire;
use crate::browser::tls13::Refusal;

/*
 * One pass: the chain, the CertificateVerify and the Finished MAC are checked
 * once over the messages decrypted as they arrived, and the request is sealed
 * only after all of them passed. The flight used to be verified twice, and a
 * refusal decrypted it a third time to look for an alert the handshake state
 * already holds. A handshake that ran is never run again.
 */
/// Answer a finished flight: send the client Finished and the request, or
/// stop with why the server was not answered.
pub(super) fn verify_and_send<W: Wire>(w: &mut W, f: &mut Fetch, end: usize) {
    let req = request::request(f, w.wall_ms());
    let host = f.url.host.as_bytes();
    let Some(tls) = f.tls.as_mut() else { return f.stop("tls handshake failed") };
    let Some(hs) = tls.hs.as_ref() else { return f.stop("tls handshake failed") };
    match hs.answer(host, tls.now, req.as_bytes()) {
        Ok(answer) => {
            /* The request rides with the Finished; counted before the send,
             * since one that failed part way may have left. */
            f.requested = true;
            if w.send(f.handle, &answer.flight).is_err() {
                return f.stop("send failed");
            }
            /* What followed the Finished in the same read is response. */
            f.buf = tls.flight.split_off(end.min(tls.flight.len()));
            tls.settle(answer.app);
            f.tx_seq = 0;
            f.phase = Phase::ReadBody;
        }
        Err(Refusal::Alert(description)) => {
            f.tls_alert = Some(description);
            f.stop("tls handshake refused");
        }
        Err(Refusal::Unverified) => {
            /* Read after the refusal, for the page; it decides nothing. */
            let why = TlsWhy::Cert(hs.cert_problem(host, tls.now), tls.now);
            f.tls_why = Some(why);
            f.stop("tls handshake refused");
        }
        Err(Refusal::Incomplete | Refusal::Seal) => f.stop("tls handshake failed"),
    }
}
