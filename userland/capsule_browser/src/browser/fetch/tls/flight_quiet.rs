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

//! What a gap in the flight means once nothing more is arriving.

use crate::browser::fetch::budget;
use crate::browser::fetch::types::{Fetch, Phase};

/// Count one quiet tick, and decide whether the flight is done or lost.
pub(super) fn quiet(f: &mut Fetch, settled: bool, have: usize) {
    f.idle = f.idle.wrapping_add(1);
    if settled && f.idle >= budget::flight_settle() {
        // Believed on a quiet gap rather than because the flight said it was
        // done. Over the mixnet the rest can still be on its way, so this is
        // the case worth being able to tell apart.
        super::trace::flight(b"settled", have, f.idle);
        f.phase = Phase::TlsVerify;
    } else if f.idle >= budget::hs_wait() {
        super::trace::flight(b"abandoned", have, f.idle);
        f.error = Some("tls handshake failed");
        f.phase = Phase::Error;
    }
}
