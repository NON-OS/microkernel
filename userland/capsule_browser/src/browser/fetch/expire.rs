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

use super::tls;
use super::types::{Fetch, Phase};
use crate::browser::http::response::has_headers;

/// Out of time. A response whose headers are in is kept as far as it got;
/// anything else stops with `reason`, and a handshake that never verified
/// is not verified now just to look for headers.
pub(super) fn expire(f: &mut Fetch, reason: &'static str) {
    if f.phase != Phase::ReadBody {
        return f.stop(reason);
    }
    let headed = match f.tls.is_some() {
        true => tls::plain(f).is_some_and(has_headers),
        false => has_headers(&f.buf),
    };
    match (headed, f.tls.is_some()) {
        (false, _) => f.stop(reason),
        (true, true) => f.phase = Phase::Decrypt,
        (true, false) => f.phase = Phase::Done,
    }
}
