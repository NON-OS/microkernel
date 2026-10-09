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

use crate::browser::fetch::types::{Fetch, Phase, TlsCtx};
use crate::browser::fetch::wire::Wire;
use crate::browser::tls13;

/// Send the ClientHello on a connection that has just been accepted.
pub(in crate::browser::fetch) fn hello<W: Wire>(w: &mut W, f: &mut Fetch) {
    let Some(cf) = tls13::client_flight(f.url.host.as_bytes()) else {
        return f.stop("tls init failed");
    };
    if w.send(f.handle, &cf.record).is_err() {
        return f.stop("send failed");
    }
    f.tls = Some(TlsCtx::new(cf, w.rtc_now()));
    f.phase = Phase::TlsFlight;
}
