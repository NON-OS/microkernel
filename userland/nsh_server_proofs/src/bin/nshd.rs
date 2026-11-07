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

//! A development nsh/1 host over real TCP, built from the proofs' honest
//! fixture kernel. It serves the real attested, post-quantum handshake to the
//! real `nsh_client`, one connection at a time. This is NOT a production host:
//! it uses throwaway test keys and a software attestation fixture, so it must
//! only ever listen on a trusted, isolated network (e.g. a QEMU/KVM guest or a
//! lab box). Usage: `nshd [HOST:PORT]` (default 0.0.0.0:2222, or $NSHD_ADDR).
//! With $NSHD_RECORDS naming a directory, every session's record and its
//! checkpoints are kept there (`nsh_record_store`).

use nsh_record_store::Store;
use nsh_server::{EndReason, Event as S};
use nsh_server_proofs::kit::demo::serve::serve_conn;
use std::net::TcpListener;

fn main() {
    let addr = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("NSHD_ADDR").ok())
        .unwrap_or_else(|| "0.0.0.0:2222".to_string());
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("nshd: bind {addr}: {e}");
        std::process::exit(1);
    });
    let store = std::env::var("NSHD_RECORDS").ok().map(|d| {
        Store::open(&d).unwrap_or_else(|e| {
            eprintln!("nshd: records {d}: {e}");
            std::process::exit(1);
        })
    });
    if let Some(st) = &store {
        eprintln!("nshd: keeping session records in {}", st.dir().display());
    }
    let bound = listener.local_addr().map(|a| a.to_string()).unwrap_or(addr);
    eprintln!("nshd: nsh/1 attested host (fixture kernel) listening on {bound}");
    eprintln!("nshd: suite 1 - X-Wing (X25519+ML-KEM-768) KEX; Ed25519+ML-DSA-65 host signature");
    for conn in listener.incoming() {
        let sock = match conn {
            Ok(s) => s,
            Err(e) => {
                eprintln!("nshd: accept: {e}");
                continue;
            }
        };
        let peer = sock.peer_addr().map(|a| a.to_string()).unwrap_or_default();
        eprintln!("nshd: <- {peer} connected; running HOST_PROOF / USER_AUTH handshake");
        let events = serve_conn(sock, store.as_ref());
        let admitted = events.iter().any(|e| matches!(e, S::Admit(_)));
        let exited = events.iter().any(|e| matches!(e, S::Closed(EndReason::Exit)));
        eprintln!("nshd: -> {peer} admitted={admitted} clean_exit={exited}");
    }
}
