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

/*
 * A byte stream to a host over a route: a socket through net.sockets when
 * Direct is the choice, a tunnel through net.socks5 or net.anon otherwise.
 * A route that is down is refused here with its reason, and a stream that
 * cannot be opened on the chosen route is refused with the network's; no
 * failure here is ever answered by trying another way.
 */

use nonos_libc::mk_uptime_ms;
use nonos_socket::{SocketError, TcpStream};

use crate::ipc::Ipc;
use crate::opening::Opening;
use crate::pick::Route;
use crate::refusal::Proxy;
use crate::slice::Slice;
use crate::tunnel::Tunnel;

pub enum RouteStream {
    Direct(TcpStream),
    Proxied(Tunnel<Ipc>),
}

impl RouteStream {
    pub fn connect(route: Route, host: &str, port: u16) -> Result<RouteStream, &'static str> {
        let (route, anon) = for_host(route, host);
        match route {
            Route::Direct => {
                TcpStream::connect(host, port).map(RouteStream::Direct).map_err(direct_refusal)
            }
            Route::Nym(p) => {
                Tunnel::open(Ipc::new(p), Proxy::Nym, host, port).map(RouteStream::Proxied)
            }
            Route::Anon(p) => Tunnel::open(Ipc::new(p), anon, host, port).map(RouteStream::Proxied),
            Route::Down(why) => Err(why),
        }
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), &'static str> {
        match self {
            RouteStream::Direct(s) => s.write_all(data).map_err(|_| DIRECT_BROKE),
            RouteStream::Proxied(t) => t.write_all(data),
        }
    }

    /* What has arrived; zero when nothing has yet. */
    pub fn read(&mut self, into: &mut [u8]) -> Result<usize, &'static str> {
        match self {
            RouteStream::Direct(s) => s.read(into).map_err(|_| DIRECT_BROKE),
            RouteStream::Proxied(t) => t.read(into),
        }
    }

    /*
     * What has arrived, waiting up to `wait_ms` for the first of it. With
     * no wait it is one read, as a socket read always was.
     */
    pub fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        match self {
            RouteStream::Proxied(t) => t.read_wait(into, wait_ms),
            RouteStream::Direct(s) => {
                let wait = i64::try_from(wait_ms).unwrap_or(i64::MAX);
                let until = mk_uptime_ms().saturating_add(wait);
                loop {
                    let n = s.read(into).map_err(|_| DIRECT_BROKE)?;
                    if n > 0 || mk_uptime_ms() >= until {
                        return Ok(n);
                    }
                }
            }
        }
    }

    /*
     * The far end finished and all it sent has been read. A direct socket
     * does not say, and its readers keep their own quiet windows.
     */
    pub fn ended(&self) -> bool {
        match self {
            RouteStream::Direct(_) => false,
            RouteStream::Proxied(t) => t.ended(),
        }
    }
}

/*
 * The route a host is reached on, and which of net.anon's two ways serves
 * it: an .anyone service goes through net.anon whatever the default.
 */
fn for_host(route: Route, host: &str) -> (Route, Proxy) {
    let service = crate::pick::is_anyone(host);
    let route = if service {
        crate::pick::for_host(route, host, nonos_socket::lookup(b"net.anon"))
    } else {
        route
    };
    let anon = match (service, crate::pick::is_short_anyone(host)) {
        (true, true) => Proxy::AnyoneName,
        (true, false) => Proxy::AnyoneService,
        (false, _) => Proxy::Anyone,
    };
    (route, anon)
}

/*
 * A stream through a proxy, opened in slices for a caller that must not
 * wait (`opening.rs`). Direct is the caller's own socket to make, and a
 * route that is down is refused with its reason, as `connect` refuses it.
 */
pub struct RouteOpening(Opening<Ipc>);

impl RouteOpening {
    /* No call made through it waits longer than `slice_ms`. */
    pub fn begin(
        route: Route,
        host: &str,
        port: u16,
        slice_ms: u64,
    ) -> Result<RouteOpening, &'static str> {
        let (route, anon) = for_host(route, host);
        let (port_of, proxy) = match route {
            Route::Nym(p) => (p, Proxy::Nym),
            Route::Anon(p) => (p, anon),
            Route::Direct => return Err(NOT_PROXIED),
            Route::Down(why) => return Err(why),
        };
        Opening::begin(Ipc::new(port_of), proxy, host, port, slice_ms).map(RouteOpening)
    }

    /* One step, waiting at most `slice_ms` on the proxy. */
    pub fn step(&mut self, slice_ms: u64) -> Slice {
        self.0.step(slice_ms)
    }

    /* The stream, once a step said Done. */
    pub fn into_stream(self) -> RouteStream {
        RouteStream::Proxied(self.0.into_tunnel())
    }
}

impl RouteStream {
    /*
     * What has arrived, waiting at most `slice_ms` on the proxy. A direct
     * socket read does not wait.
     */
    pub fn read_slice(&mut self, into: &mut [u8], slice_ms: u64) -> Result<usize, &'static str> {
        match self {
            RouteStream::Proxied(t) => t.read_slice(into, slice_ms),
            RouteStream::Direct(s) => s.read(into).map_err(|_| DIRECT_BROKE),
        }
    }

    /*
     * Carry the front of `data`, waiting at most `slice_ms` on the proxy.
     * How many bytes went: zero means offer the same bytes again later. A
     * direct socket takes them all in one write.
     */
    pub fn write_slice(&mut self, data: &[u8], slice_ms: u64) -> Result<usize, &'static str> {
        match self {
            RouteStream::Proxied(t) => t.write_slice(data, slice_ms),
            RouteStream::Direct(s) => {
                s.write_all(data).map(|()| data.len()).map_err(|_| DIRECT_BROKE)
            }
        }
    }
}

const NOT_PROXIED: &str = "a direct connection is not opened through a proxy";

const DIRECT_BROKE: &str = "the direct connection failed";

fn direct_refusal(e: SocketError) -> &'static str {
    match e {
        SocketError::NoService => "net.sockets is not running, so there is no direct network",
        SocketError::BadHost => "the host name is empty or too long",
        _ => "the host could not be reached directly",
    }
}
