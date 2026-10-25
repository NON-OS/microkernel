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

//! The client fed arbitrary server bytes; the first byte picks the stage.
//! 0 and 4: raw bytes after each hello. 1, 2 and 3: sealed frames (a u16
//! length, then type and body) after `SERVER_HELLO`, `HOST_PROOF` and
//! `AUTH_OK`. 5: a `HOST_PROOF` signed over fuzzed fields, answered with
//! `USER_AUTH` exactly when the oracle says every host check passes.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_client::Refusal;
use nsh_client_proofs::kit::fuzz_run::host_run;
use nsh_client_proofs::kit::pair::Pair;
use nsh_wire::Kind;

fuzz_target!(|data: &[u8]| {
    let Some((stage, rest)) = data.split_first() else { return };
    if stage % 6 == 5 {
        let run = host_run(rest);
        assert_eq!(run.credential, run.oracle.all(), "{:?}", run.oracle);
        return;
    }
    let mut p = Pair::good();
    let mut user_auths = 0;
    match stage % 6 {
        0 => {
            p.client.take_output();
            let _ = p.feed(rest);
        }
        1 | 4 => p.to_proof(),
        2 => user_auths += usize::from(!p.to_auth().is_empty()),
        _ => user_auths += usize::from(p.login().len() == 2),
    }
    if matches!(stage % 6, 1..=3) {
        let mut r = rest;
        while let [a, b, tail @ ..] = r {
            let n = usize::from(u16::from_le_bytes([*a, *b])).min(tail.len());
            let inner = &tail[..n];
            r = &tail[n..];
            let f = if inner.is_empty() { vec![] } else { p.server.seal_inner(inner) };
            if p.feed(&f).is_err() {
                break;
            }
        }
    } else if stage % 6 == 4 {
        let _ = p.feed(rest);
    }
    let out = p.client.take_output();
    if let Some(frames) = p.server.open(&out) {
        user_auths += frames.iter().filter(|f| f.0 == Kind::UserAuth.byte()).count();
    }
    assert!(user_auths == 0 || p.client.attester().said_ok.get(), "a credential before the check");
    assert!(user_auths <= 1, "one USER_AUTH at most");
    if matches!(stage % 6, 0 | 4) {
        assert_eq!(p.client.attester().calls.get(), 0, "nothing sealed can open");
    }
    if p.client.is_over() {
        assert_eq!(p.feed(&[0]), Err(Refusal::Ended));
    }
});
