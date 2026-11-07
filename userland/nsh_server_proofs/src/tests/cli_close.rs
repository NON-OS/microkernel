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

//! `nsh` against a host that takes the login, then never ends the session
//! once the client closes it, as a host whose shell does not answer the
//! stop: the client gives up after --close-wait and exits 255, with the
//! line cli_explain checks for Ended::Unended, instead of waiting for a
//! record that never comes.

use super::cli::nsh;
use super::cli_files::files;
use super::loopback_host::{urandom, Clock};
use crate::kit::host::{config, Kernel, Keys};
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use nsh_server::{Event as S, Server};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::time::{Duration, Instant};

/// Serve one connection, beginning the session it admits and ignoring its
/// stop, until the client hangs up; whether the client asked to close.
fn never_ends(listener: &TcpListener) -> bool {
    let clock = Clock::start();
    let mut s =
        Server::new(config(), Kernel::honest(), Keys::default(), Table::shipped(), clock.now())
            .expect("config");
    let (mut sock, _) = listener.accept().expect("accept");
    sock.set_read_timeout(Some(Duration::from_millis(20))).expect("timeout");
    let (mut closed, mut buf) = (false, vec![0u8; 65_536]);
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        match sock.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => s.feed(&buf[..n], clock.now(), &mut urandom),
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(_) => break,
        }
        for e in s.events() {
            match e {
                S::Admit(a) => s.begin(a.caps, clock.now(), &mut urandom).expect("begin"),
                S::Stop(_) => closed = true,
                _ => {}
            }
        }
        if sock.write_all(&s.take_output()).is_err() {
            break;
        }
    }
    closed
}

#[test]
fn nsh_gives_up_on_a_host_that_never_ends_a_closed_session() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let host = std::thread::spawn(move || never_ends(&listener));
    let (dir, mut args) = files("unended", &addr, &GrantB::default());
    args.extend(["--close-wait".into(), "1".into()]);
    let began = Instant::now();
    let (status, out) = nsh(&args, &[b"\x1d"]);
    let took = began.elapsed();
    assert!(host.join().expect("the host"), "the client closed the session");
    assert_eq!(status, 255, "{out}");
    assert!(took >= Duration::from_secs(1), "{took:?}");
    assert!(took < Duration::from_secs(10), "gave up in time, not {took:?}");
    let kept = std::fs::read_dir(dir.join("records")).map(|d| d.count()).unwrap_or(0);
    assert_eq!(kept, 0, "no record is kept for a session the host never ended");
}
