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

//! Connecting: the user's files read, the fleet's policy and releases built,
//! fresh randomness, the host's address, the terminal in raw mode, and then
//! the session.

use super::explain::ended;
use super::report::Report;
use super::{args, files, session, term, urandom};
use crate::{Client, NonosAttester};
use alloc::string::String;
use alloc::vec::Vec;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub(super) fn connect(
    a: &args::Args,
    keys: Receiver<Vec<u8>>,
    out: &mut dyn Write,
) -> Result<i32, String> {
    let (cfg, policy, fleet, escalate) = super::load::load(a)?;
    let (mut seed, mut random) = ([0u8; 32], [0u8; 32]);
    if !urandom(&mut seed) || !urandom(&mut random) {
        return Err("no randomness".into());
    }
    let user = files::user_key(&a.key)?;
    let mut c = Client::new(
        cfg,
        NonosAttester { policy: fleet },
        user,
        nsh_crypto::Secret::take(&mut seed),
        random,
    )
    .map_err(|e| {
        alloc::format!("refused before connecting: {} [{e:?}]", super::explain::refusal(&e))
    })?;
    let addr =
        a.connect.to_socket_addrs().map_err(|e| alloc::format!("{}: {e}", a.connect))?.next();
    let addr = addr.ok_or_else(|| alloc::format!("{}: no address", a.connect))?;
    let mut sock = TcpStream::connect_timeout(&addr, Duration::from_secs(10))
        .map_err(|e| alloc::format!("{addr}: {e}"))?;
    let _ = sock.set_nodelay(true);
    let raw = term::Raw::enter();
    let window = raw.is_raw().then(term::size).flatten();
    let start = session::Start {
        window,
        escalate,
        handshake: Duration::from_secs(a.handshake),
        close_wait: Duration::from_secs(a.close_wait),
    };
    let mut report = Report::new(out, &policy, a.records.clone());
    let done = session::run(&mut c, &mut sock, &keys, &start, &mut report);
    drop(raw);
    done.map_err(|e| ended(&e))
}
