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

//! `CLIENT_HELLO` and the host's proof: only `VERSION` and `BUSY` can be
//! told before keys exist; everything else broken there is closed without
//! a word. A host that cannot attest or sign says `BUSY` or nothing, and
//! never sends a proof the client would refuse.

use crate::kit::host::{config, Kernel, Keys};
use crate::kit::raw_run::RawRun;
use crate::kit::rng::dry;
use crate::kit::roles::Table;
use crate::kit::roles_build::NOW;
use nsh_server::{BadConfig, Config, Server, MIN_RECORD};
use nsh_wire::Reason;

#[test]
fn the_handshake_has_a_deadline() {
    let mut r = RawRun::new();
    r.server.tick(NOW + 29_999, &mut dry);
    assert!(!r.server.is_over());
    r.server.tick(NOW + 30_000, &mut dry);
    assert!(r.server.is_over() && r.server.take_output().is_empty());
    let mut r = RawRun::proved();
    r.server.tick(NOW + 30_000, &mut dry);
    let out = r.server.take_output();
    assert_eq!(r.raw.open(&out), [(0x7F, Reason::Limit.code().to_le_bytes().to_vec())]);
}

#[test]
fn bytes_after_the_end_are_dropped() {
    let mut r = RawRun::new();
    let mut hello = r.raw.hello.clone();
    hello[9] = 9;
    r.send(&hello);
    assert!(r.send(&[1, 2, 3, 4, 5]).is_empty());
    assert_eq!(r.refused().len(), 1);
}

#[test]
fn a_config_the_server_cannot_keep_is_refused() {
    let bad = [
        Config { idle_s: 0, ..config() },
        Config { idle_s: 86_401, ..config() },
        Config { handshake_ms: 0, ..config() },
        Config { max_record: MIN_RECORD - 1, ..config() },
        Config { max_record: (64 << 20) + 1, ..config() },
        Config { enrol: Vec::new(), ..config() },
        Config { enrol: vec![0; 100], ..config() },
    ];
    for cfg in bad {
        let s = Server::new(cfg, Kernel::honest(), Keys::default(), Table::shipped(), NOW);
        assert_eq!(s.err(), Some(BadConfig));
    }
}
