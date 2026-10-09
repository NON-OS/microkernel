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

//! A connection to the RPC host opened a step at a time, on the same terms
//! as `Link::open`. Through the Nym mixnet or the Anyone network each step
//! asks the proxy once, for a slice. Direct takes its three calls to the
//! local stack (the name, the socket, the connect) one per step; each is
//! the bounded call it always was.

use nonos_route_link::{Route, RouteOpening, Slice};

use super::super::constants::{SERVICE_DNS, SERVICE_SOCKETS};
use super::super::link::{Link, Reach};
use super::SLICE_MS;

const NO_DIRECT: &str = "net.dns or net.sockets is not running, so there is no direct network";
const NO_NAME: &str = "the RPC host's name did not resolve";
const NO_SOCKET: &str = "net.sockets gave no socket";
const NO_ANSWER: &str = "the RPC host did not answer";

pub enum Direct {
    Resolve,
    Socket { sockets: u32, ip: [u8; 4] },
    Connect { sockets: u32, ip: [u8; 4], handle: u32 },
}

pub enum Opening {
    Direct(Direct),
    Routed {
        opening: RouteOpening,
        patience_ms: u64,
    },
    /// A route that is down, said on the first step.
    Down(&'static str),
}

impl Opening {
    pub fn begin(route: Route, host: &'static str) -> Opening {
        match route {
            Route::Direct => Opening::Direct(Direct::Resolve),
            Route::Nym(_) | Route::Anon(_) => match RouteOpening::begin(route, host, 443, SLICE_MS)
            {
                Ok(opening) => Opening::Routed { opening, patience_ms: route.patience_ms() },
                Err(why) => Opening::Down(why),
            },
            Route::Down(why) => Opening::Down(why),
        }
    }

    /// One step: the link once open, None while it is opening, or how far
    /// it got and why it failed.
    pub fn step(&mut self) -> Result<Option<Link>, Reach> {
        let fail = |resolve, socket, why| Reach { resolve, socket, why };
        match self {
            Opening::Down(why) => Err(fail(false, false, why)),
            Opening::Routed { opening, patience_ms } => match opening.step(SLICE_MS) {
                Slice::Waiting => Ok(None),
                Slice::Failed(why) => Err(fail(false, false, why)),
                Slice::Done => {
                    let patience_ms = *patience_ms;
                    let done = core::mem::replace(self, Opening::Down(NO_ANSWER));
                    let Opening::Routed { opening, .. } = done else {
                        return Err(fail(false, false, NO_ANSWER));
                    };
                    let stream = opening.into_stream();
                    Ok(Some(Link::Routed { stream, patience_ms }))
                }
            },
            Opening::Direct(stage) => step_direct(stage),
        }
    }
}

fn step_direct(stage: &mut Direct) -> Result<Option<Link>, Reach> {
    let fail = |resolve, socket, why| Reach { resolve, socket, why };
    match *stage {
        Direct::Resolve => {
            let dns = super::super::lookup::lookup(SERVICE_DNS);
            let sockets = super::super::lookup::lookup(SERVICE_SOCKETS);
            if dns == 0 || sockets == 0 {
                return Err(fail(false, false, NO_DIRECT));
            }
            let ip = super::super::resolve_eth::resolve_eth(dns)
                .map_err(|_| fail(false, false, NO_NAME))?;
            *stage = Direct::Socket { sockets, ip };
            Ok(None)
        }
        Direct::Socket { sockets, ip } => {
            let handle = super::super::socket_open::socket_open(sockets)
                .map_err(|_| fail(true, false, NO_SOCKET))?;
            *stage = Direct::Connect { sockets, ip, handle };
            Ok(None)
        }
        Direct::Connect { sockets, ip, handle } => {
            if super::super::socket_connect::socket_connect(sockets, handle, ip, 443).is_err() {
                /* Closed as the stage is dropped. */
                return Err(fail(true, true, NO_ANSWER));
            }
            /* The link owns the socket now; this stage no longer does. */
            *stage = Direct::Resolve;
            Ok(Some(Link::Direct { sockets, handle }))
        }
    }
}

impl Drop for Direct {
    /* A socket opened for a fetch given up before it connected. */
    fn drop(&mut self) {
        if let Direct::Connect { sockets, handle, .. } = *self {
            let _ = super::super::socket_close::socket_close(sockets, handle);
        }
    }
}
