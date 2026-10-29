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

//! Whether an internet socket may listen: with NET_LISTEN_LO wherever bind
//! put it; without it, only at the one address SERVE_ONE names, where the
//! first listen is served to the field's capsule before the socket listens
//! (served.rs), and a second listener is EADDRINUSE.

use nonos_linux_policy::mask::NET_LISTEN_LO;
use nonos_linux_policy::ServeOne;

use crate::linux::abi::errno;
use crate::linux::app;
use crate::linux::guest::Guest;
use crate::linux::net::sock::{self, Addr, Domain, Proto};

/// Ok to go on with the listen on `id`, or the errno that refuses it.
pub fn gate(guest: &Guest, id: u32) -> Result<(), u64> {
    let got = sock::with(|t| t.get(id).map(|s| (s.domain, s.proto == Proto::Stream, s.local)));
    let Some((Domain::Inet, stream, local)) = got else {
        return Ok(());
    };
    let at = |f: &(ServeOne, Addr)| app::serve_at(guest, stream, f.1.ip, f.1.port);
    match guest.app.policy.serve_one().zip(local).filter(at) {
        Some((field, at)) => served(id, field, at),
        None if app::granted(guest, NET_LISTEN_LO) => Ok(()),
        None => Err(app::refuse("listen", "NET_LISTEN_LO", errno::EACCES)),
    }
}

/// SO_REUSEPORT shares no served listener: one capsule, one listener.
fn served(id: u32, field: ServeOne, at: Addr) -> Result<(), u64> {
    if sock::with(|t| t.iter().any(|(i, s)| i != id && s.listening && s.local == Some(at))) {
        return Err(errno::fail(errno::EADDRINUSE));
    }
    super::served::listen(field, at)
}
