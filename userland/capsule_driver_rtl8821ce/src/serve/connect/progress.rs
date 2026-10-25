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

//! The handshake progress line.

use crate::assoc::{self};
use crate::status;

// The handshake progress, one line a join: frames sent and received, data and
// EAPOL frames seen, deauthentications, and the state reached, so a failed join
// names where it stopped (`log rtl8821ce`). The panel carries the same numbers.
pub(super) fn log_progress(report: &assoc::Report) {
    // Always said, one line a join: where a join stops is the first thing a
    // failed one on a real machine needs, and nothing secret is in it.
    status::line(b"[rtl8821ce] connect: sent=");
    status::number(report.sent);
    status::line(b" recv=");
    status::number(report.recv);
    status::line(b" data=");
    status::number(report.data);
    status::line(b" to_us=");
    status::number(report.to_us);
    status::line(b" eapol=");
    status::number(report.eapol);
    status::line(b" deauth=");
    status::number(report.deauth);
    status::line(b" state=");
    status::number(report.state as u32);
    status::line(b" probe=0x");
    status::hex16((report.probe >> 16) as u16);
    status::hex16(report.probe as u16);
    status::line(b"\n");
}
