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

use crate::browser::fetch::types::Fetch;
use crate::browser::fetch::wire::Wire;
use crate::browser::net::drain::drain;

/* A SOCKS reply is at most 262 bytes; nothing else arrives before it. */
const SOCKS_CAP: usize = 512;
const SOCKS_READ_MS: i64 = 5;

/// Take what the proxy has answered so far. How long it may take to answer
/// at all is the fetch deadline's to decide.
pub fn recv_some<W: Wire>(w: &mut W, f: &mut Fetch) {
    let read = drain(w, f.handle, &mut f.socks, SOCKS_CAP, SOCKS_READ_MS);
    if read.got > 0 {
        f.progress_ms = w.now_ms();
        f.received += read.got;
    }
}
