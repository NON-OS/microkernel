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

use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::wire::Wire;

pub(in crate::browser::fetch) fn send_req<W: Wire>(w: &mut W, f: &mut Fetch) {
    let req = super::request::request(f, w.wall_ms());
    /* Counted before the send: one that failed part way may have left. */
    f.requested = true;
    if w.send(f.handle, req.as_bytes()).is_err() {
        return f.stop("send failed");
    }
    f.buf.clear();
    f.phase = Phase::ReadBody;
}
