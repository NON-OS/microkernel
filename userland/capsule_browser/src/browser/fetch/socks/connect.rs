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

use crate::browser::fetch::socks::{next_phase, recv_some, response_len};
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::wire::Wire;
use crate::browser::net::mixnet;

pub fn connect<W: Wire>(w: &mut W, f: &mut Fetch) {
    recv_some::recv_some(w, f);
    if f.socks.len() < 5 || matches!(f.phase, Phase::Error) {
        return;
    }
    let Some(need) = response_len::response_len(&f.socks) else {
        return f.stop("bad socks response");
    };
    if f.socks.len() < need {
        return;
    }
    if f.socks[0] != 0x05 || f.socks[1] != 0x00 || f.socks[2] != 0x00 {
        /*
         * The proxy already said which step refused, so report that rather
         * than the bare fact of refusal. Each of these is a different thing
         * to go and look at, and on a machine with no console this line is
         * the only place the difference shows.
         */
        return f.stop(mixnet::chosen().refused(f.socks[1]));
    }
    f.socks.clear();
    f.phase = next_phase::next_phase(f.url.scheme);
}
