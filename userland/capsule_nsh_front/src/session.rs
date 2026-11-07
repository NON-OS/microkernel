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

//! Serving one connection with the sans-IO `nsh_server`. The socket's bytes
//! are fed in; what the server hands back is sent; its events drive the
//! shell. The host keys stay in nsh.attest, which signs for the server; the
//! enrolment, role table and revocation list are the machine's own records,
//! read for this connection; the attestation is the live machine's. A host
//! that cannot serve tells the client why in plaintext (unready.rs) and
//! says the whole reason on the serial console. The loop ends when the protocol is over (an exit, a timeout, a
//! broken frame) or the socket is gone, and the record's last bytes go out
//! before the socket closes. While it runs, every other connection the
//! listener takes is told BUSY (newcomers.rs); a connection the host cannot
//! start a session for is told BUSY too, never closed without a word.

use crate::attest::AttestBridge;
use crate::host::HostBridge;
use crate::limits::{HANDSHAKE_MS, IDLE_S, MAX_RECORD, SESSION_MAX_MS};
use crate::marks::Marks;
use crate::newcomers;
use crate::relay_said::Fault;
use crate::relay_said::ST_NO_TPM;
use crate::shell::Shell;
use crate::sock::{Net, Recv};
use crate::turn_away::Away;
use crate::unready::Unready;
use crate::{clock, records, rng};
use nonos_libc::{mk_debug, mk_idle_ms};
use nsh_roles::PolicyError;
use nsh_server::{busy, turned_away, Config, RoleStore, Server};

/// The server, specialised to this capsule's host.
pub type Host = Server<AttestBridge, HostBridge, RoleStore>;

/// How long to park when a turn did no work, so an idle session costs no core.
const IDLE_MS: u64 = 5;

/// Serve `child` to completion, then close it, telling every connection
/// `listener` takes meanwhile BUSY. Returns whether `listener` is still
/// listening, as far as net.sockets said.
pub fn serve(
    net: &mut Net,
    child: u32,
    attest_port: u32,
    listener: u32,
    marks: &mut Marks,
) -> bool {
    let mut away = Away::new();
    match start(attest_port, marks) {
        Ok(mut s) => {
            run(net, child, listener, &mut s, &mut away);
            let last = s.take_output();
            if !last.is_empty() {
                let _ = net.send(child, &last);
            }
            net.close(child);
        }
        Err(why) => {
            let text = why.line();
            let _ = mk_debug(text.as_ptr(), text.len());
            let answer = turned_away(why.reason()).unwrap_or_else(busy);
            newcomers::tell(net, &mut away, child, newcomers::uptime(), &answer);
        }
    }
    newcomers::let_go(net, &mut away);
    away.listening()
}

/// A server for one connection, or why the host cannot serve anyone now.
fn start(attest_port: u32, marks: &mut Marks) -> Result<Host, Unready<PolicyError>> {
    let signer = HostBridge::fetch(attest_port).map_err(|f| match f {
        Fault::Status(ST_NO_TPM) => Unready::NoTpm,
        _ => Unready::NoAnswer,
    })?;
    let held = records::fetch(attest_port)?;
    let roles = RoleStore::new(held.table, held.anchor, marks.serial(), held.list)
        .map_err(Unready::Table)?;
    if !marks.admit(roles.serial(), roles.issued_ms()) {
        return Err(Unready::Older);
    }
    let cfg = Config {
        enrol: held.enrol,
        handshake_ms: HANDSHAKE_MS,
        idle_s: IDLE_S,
        ckpt_every: 0,
        max_record: MAX_RECORD,
    };
    let kernel = AttestBridge::new(attest_port);
    Server::new(cfg, kernel, signer, roles, clock::now()).map_err(|_| Unready::Enrol)
}

fn run(net: &mut Net, child: u32, listener: u32, s: &mut Host, away: &mut Away) {
    let deadline = clock::now().saturating_add(SESSION_MAX_MS);
    let mut shell = Shell::new();
    let mut seq = 1u32;
    while !s.is_over() && clock::now() < deadline {
        newcomers::round(net, listener, away);
        let mut progressed = false;
        let mut rand = rng::fill;
        match net.recv(child, seq) {
            Recv::Data(d) => {
                seq = seq.wrapping_add(1).max(1);
                s.feed(&d, clock::now(), &mut rand);
                progressed = true;
            }
            Recv::Idle => {}
            Recv::Gone => break,
        }
        s.tick(clock::now(), &mut rand);
        let mut todo = s.events();
        while !todo.is_empty() {
            for e in &todo {
                shell.act(s, e);
            }
            progressed = true;
            todo = s.events();
        }
        let out = s.take_output();
        if !out.is_empty() {
            if !net.send(child, &out) {
                break;
            }
            progressed = true;
        }
        if !progressed {
            let _ = mk_idle_ms(IDLE_MS);
        }
    }
}
