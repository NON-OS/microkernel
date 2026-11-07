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

//! The host side of the loopback: one connection accepted, the server fed
//! from the socket, its output written back, the clock ticked, and each
//! line of input run by `/bin/sh -c` with its output sent and recorded.

pub use super::loopback_clock::{urandom, Clock};
use super::loopback_shell::act;
use crate::kit::host::{config, Kernel, Keys};
use crate::kit::roles::Table;
use nsh_server::{Event as S, HostSigner, Roles, Server};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::time::{Duration, Instant};

/// Serve one connection; every event the server reported.
pub fn serve(listener: &TcpListener, kernel: Kernel, table: Table) -> Vec<S> {
    let clock = Clock::start();
    let s = Server::new(config(), kernel, Keys::default(), table, clock.now()).expect("config");
    serve_on(listener, s, &clock)
}

/// Serve one connection with `s`, on `clock`; every event it reported.
pub fn serve_on<K: HostSigner, R: Roles>(
    listener: &TcpListener,
    mut s: Host<K, R>,
    clock: &Clock,
) -> Vec<S> {
    let (mut sock, _) = listener.accept().expect("accept");
    sock.set_read_timeout(Some(Duration::from_millis(20))).expect("timeout");
    let (mut events, mut line, mut buf) = (Vec::new(), Vec::new(), vec![0u8; 65_536]);
    let deadline = Instant::now() + Duration::from_secs(30);
    while !s.is_over() && Instant::now() < deadline {
        match sock.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => s.feed(&buf[..n], clock.now(), &mut urandom),
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(e) => panic!("read: {e}"),
        }
        s.tick(clock.now(), &mut urandom);
        let mut todo = s.events();
        while !todo.is_empty() {
            for e in todo {
                act(&mut s, &e, &mut line, clock);
                events.push(e);
            }
            todo = s.events();
        }
        sock.write_all(&s.take_output()).expect("write");
    }
    sock.write_all(&s.take_output()).expect("write");
    events
}

pub(super) type Host<K = Keys, R = Table> = Server<Kernel, K, R>;
