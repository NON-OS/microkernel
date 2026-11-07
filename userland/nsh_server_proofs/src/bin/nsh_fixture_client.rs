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

//! A matching demo client for `nshd`: the real `nsh_client` and the real
//! `NonosAttester`, carrying the fixture user key, grant and client policy
//! that pair with `nshd`'s fixture host. It verifies the host's attestation
//! before sending any credential; a host that fails is refused. Use it to
//! watch a genuine remote attested post-quantum handshake succeed.
//! Usage: `nsh-fixture-client HOST:PORT [shell command ...]`.

use nsh_server_proofs::kit::demo::client::run_client;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut args = std::env::args().skip(1);
    let target = args.next().unwrap_or_else(|| {
        eprintln!("usage: nsh-fixture-client HOST:PORT [shell command ...]");
        std::process::exit(2);
    });
    let rest: Vec<String> = args.collect();
    let cmd = if rest.is_empty() { "echo nsh-live-$((6*7))".to_string() } else { rest.join(" ") };
    let line = format!("{cmd}\n");
    let sock = TcpStream::connect(&target).unwrap_or_else(|e| {
        eprintln!("nsh-fixture-client: connect {target}: {e}");
        std::process::exit(1);
    });
    eprintln!("nsh-fixture-client: dialed {target}; verifying host attestation before any credential");
    let out = run_client(sock, &[line.as_bytes(), b"exit\n"]);
    if let Some(r) = out.refusal {
        eprintln!("nsh-fixture-client: REFUSED (attestation/policy): {r:?}");
        std::process::exit(1);
    }
    if !out.logged_in {
        eprintln!("nsh-fixture-client: FAILED - handshake never reached login");
        std::process::exit(1);
    }
    eprintln!("nsh-fixture-client: host attested OK; logged in. Shell output:");
    std::io::stdout().write_all(&out.output).ok();
    eprintln!("\nnsh-fixture-client: clean_exit={}", out.clean_exit);
    std::process::exit(if out.clean_exit { 0 } else { 1 });
}
