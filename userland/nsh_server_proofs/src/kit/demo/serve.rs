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

//! Serve one TCP connection with the real sans-IO `nsh_server`, fed from the
//! socket and its output written back, with `/bin/sh` behind it. The fixtures
//! (`Kernel::honest`, `Keys`, `Table::shipped`) are built here, per
//! connection, because the role table is not `Send`. With a record store, the
//! session's record and checkpoints are kept there as they grow; a store
//! that fails ends the connection, so no command runs unrecorded.

use super::shell::{act, Host};
use super::{urandom, Clock};
use crate::kit::host::{config, Kernel, Keys};
use crate::kit::roles::Table;
use nsh_record_store::{Keeper, Store};
use nsh_server::{Config, Event as S, Server};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

/// A checkpoint after this many entries when records are kept.
pub const CKPT_EVERY: u32 = 16;

/// Serve one connection to completion; returns every event the server raised.
pub fn serve_conn(mut sock: TcpStream, store: Option<&Store>) -> Vec<S> {
    sock.set_read_timeout(Some(Duration::from_millis(20))).ok();
    let clock = Clock::start();
    let cfg = match store {
        Some(_) => Config { ckpt_every: CKPT_EVERY, ..config() },
        None => config(),
    };
    let mut keeper = store.map(Keeper::new);
    let mut s: Host =
        Server::new(cfg, Kernel::honest(), Keys::default(), Table::shipped(), clock.now())
            .expect("config");
    let (mut events, mut line, mut buf) = (Vec::new(), Vec::new(), vec![0u8; 65_536]);
    let deadline = Instant::now() + Duration::from_secs(3600);
    while !s.is_over() && Instant::now() < deadline {
        match sock.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => s.feed(&buf[..n], clock.now(), &mut urandom),
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(_) => break,
        }
        s.tick(clock.now(), &mut urandom);
        let mut todo = s.events();
        while !todo.is_empty() {
            for e in todo {
                if let Some(k) = keeper.as_mut() {
                    if let Err(err) = k.keep(&e) {
                        eprintln!("nshd: the record could not be kept: {err}");
                        return events;
                    }
                }
                act(&mut s, &e, &mut line, &clock);
                events.push(e);
            }
            todo = s.events();
        }
        if sock.write_all(&s.take_output()).is_err() {
            break;
        }
    }
    let _ = sock.write_all(&s.take_output());
    events
}
