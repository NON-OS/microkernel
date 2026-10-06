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

use crate::browser::fetch::proxy_fault::PROXY_ENDED;
use crate::browser::fetch::socks::{next_phase, recv_some, response_len};
use crate::browser::fetch::socks::hold::again;
use crate::browser::fetch::types::{Fetch, Phase, Wait};
use crate::browser::fetch::wire::Wire;

/// The reply code net.socks5 and net.anon give while their network has not
/// connected (route_link's REP_NOT_YET).
const REP_NOT_YET: u8 = 0x03;

pub fn connect<W: Wire>(w: &mut W, f: &mut Fetch) {
    let closed = recv_some::recv_some(w, f);
    if matches!(f.phase, Phase::Error) {
        return;
    }
    /* Closed before its whole reply: no reply is coming. Both proxies send
     * the reply before they close, the exit's refusal included, so a close
     * without one is the proxy's own. */
    if f.socks.len() < 5 {
        if closed {
            f.stop(PROXY_ENDED);
        }
        return;
    }
    let Some(need) = response_len::response_len(&f.socks) else {
        return f.stop("bad socks response");
    };
    if f.socks.len() < need {
        if closed {
            f.stop(PROXY_ENDED);
        }
        return;
    }
    /* Not connected yet: both proxies say so while their network comes up. */
    if f.way.proxied() && f.socks[0] == 0x05 && f.socks[1] == REP_NOT_YET {
        return again(w, f, Wait::NotYet);
    }
    if f.socks[0] != 0x05 || f.socks[1] != 0x00 || f.socks[2] != 0x00 {
        /*
         * The proxy already said which step refused, so report that rather
         * than the bare fact of refusal. Each of these is a different thing
         * to go and look at, and on a machine with no console this line is
         * the only place the difference shows.
         */
        return f.stop(f.way.refused(f.socks[1]));
    }
    f.socks.clear();
    f.phase = next_phase::next_phase(f.url.scheme);
}
