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

//! Where the server's flight stands, judged from what has arrived.

use crate::browser::fetch::types::TlsCtx;
use crate::browser::tls13::{HandshakeState, Progress, Start};

pub(super) enum Flight {
    Waiting,
    /* A whole Finished is in; the flight ends at this offset. */
    Complete(usize),
    /* The server refused, with the alert it named if it named one. */
    Refused(Option<u8>),
    Failed,
}

/*
 * The flight is complete when a whole server Finished has been decrypted.
 * It used to be complete at three encrypted records, or else after fifteen
 * quiet ticks: a server that sends its whole flight as one record, as the
 * captured Cloudflare one did, was only believed after the quiet ticks, and
 * the deadline ran out first. The keys are derived once, when the
 * ServerHello is whole, and each record is opened once.
 */
pub(super) fn judge(tls: &mut TlsCtx) -> Flight {
    if tls.hs.is_none() {
        match HandshakeState::begin(&tls.cf, &tls.flight) {
            Start::Waiting => return Flight::Waiting,
            Start::Ready(state) => tls.hs = Some(state),
            Start::Alert(description) => return Flight::Refused(Some(description)),
            Start::Retry | Start::Unusable => return Flight::Failed,
        }
    }
    let Some(hs) = tls.hs.as_mut() else { return Flight::Failed };
    match hs.advance(&tls.flight) {
        Progress::Incomplete => Flight::Waiting,
        Progress::Complete(end) => Flight::Complete(end),
        Progress::Alert(description) => Flight::Refused(Some(description)),
        Progress::Broken => Flight::Failed,
    }
}
