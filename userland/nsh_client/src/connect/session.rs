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

//! One session over TCP: the client's bytes out, the host's in, keys from
//! standard input as they come, the shell's output to standard output, and
//! the client's events reported on standard error. Ctrl-], or the end of
//! standard input, closes the session; the host then sends the shell's
//! exit status and the signed record, within `start.close_wait` or the
//! client gives up on it.

use super::explain::Ended;
use super::report::Report;
use super::session_keys::keys_in;
pub use super::start::Start;
use crate::{Client, Event, NonosAttester, UserKey};
use alloc::vec::Vec;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

pub type Nsh = Client<NonosAttester, UserKey>;

/// Run to the end; the shell's exit status, or the refusal that ended it.
pub fn run(
    c: &mut Nsh,
    sock: &mut TcpStream,
    keys: &Receiver<Vec<u8>>,
    start: &Start,
    r: &mut Report,
) -> Result<i32, Ended> {
    sock.set_read_timeout(Some(Duration::from_millis(20))).map_err(|_| Ended::Lost)?;
    let (mut buf, mut status, mut typing) = (alloc::vec![0u8; 65_536], 0, false);
    // Until the login, the host has `start.handshake` to answer; after it,
    // a session may stay quiet as long as the person at it likes.
    let (deadline, mut admitted) = (Instant::now().checked_add(start.handshake), false);
    let mut closed_by: Option<Instant> = None;
    while !c.is_over() {
        sock.write_all(&c.take_output()).map_err(|_| Ended::gone(admitted))?;
        match sock.read(&mut buf) {
            Ok(0) => return Err(Ended::gone(admitted)),
            Ok(n) => c.feed(&buf[..n], &mut super::urandom)?,
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(_) => return Err(Ended::gone(admitted)),
        }
        for e in c.events() {
            match &e {
                Event::LoggedIn(_) => {
                    (typing, admitted) = (true, true);
                    start.window.map(|(cols, rows)| c.window(cols, rows)).transpose()?;
                    start.escalate.first().map(|g| c.escalate(g)).transpose()?;
                }
                Event::Escalated { .. } => {
                    let next = start.escalate.get(r.escalations() + 1);
                    next.map(|g| c.escalate(g)).transpose()?;
                }
                Event::Exit(s) => status = *s,
                _ => {}
            }
            r.event(c, &e);
        }
        if typing {
            typing = keys_in(c, keys)?;
            if !typing {
                closed_by = Instant::now().checked_add(start.close_wait);
            }
        }
        if !admitted && deadline.is_some_and(|d| Instant::now() >= d) {
            return Err(Ended::Slow(start.handshake));
        }
        if closed_by.is_some_and(|d| Instant::now() >= d) {
            return Err(Ended::Unended(start.close_wait));
        }
    }
    sock.write_all(&c.take_output()).map_err(|_| Ended::Lost)?;
    Ok(status)
}
