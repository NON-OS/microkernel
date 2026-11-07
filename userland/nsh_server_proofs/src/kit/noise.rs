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

//! The server fed arbitrary bytes, and what must hold whatever they are.
//! The first byte picks the stage: 0, raw bytes before anything; 1, sealed
//! frames (a u16 length, then type and body) once `HOST_PROOF` is sent; 2,
//! sealed frames once the session runs. Whatever arrives, the server never
//! panics; every frame it sends opens and is one the client may receive
//! then; nobody is admitted without the user's signature; nothing follows
//! the end; and a session that ends has its record signed and checking.

use super::noise_check::{allowed, record_checks};
use super::noise_plain::plain;
use super::raw_run::RawRun;
use nsh_server::Event;
use nsh_wire::TAG_LEN;

pub fn drive(data: &[u8]) {
    let Some((stage, rest)) = data.split_first() else { return };
    match stage % 3 {
        0 => plain(rest),
        n => sealed(n, rest),
    }
}

fn sealed(stage: u8, mut rest: &[u8]) {
    let mut r = if stage == 1 { RawRun::proved() } else { RawRun::session() };
    let (mut out, before) = (Vec::new(), r.events.len());
    while let [a, b, tail @ ..] = rest {
        let n = usize::from(u16::from_le_bytes([*a, *b])).min(tail.len());
        let frame = match n {
            0 => vec![0, 0, 0, 0],
            _ => r.raw.seal_inner(&tail[..n]),
        };
        rest = &tail[n..];
        out.extend(r.send(&frame));
        if r.server.is_over() {
            break;
        }
    }
    let frames = r.raw.open(&out);
    let opened: usize = frames.iter().map(|(_, b)| 4 + 1 + b.len() + TAG_LEN).sum();
    assert_eq!(opened, out.len(), "every frame the server sent opens");
    frames.iter().for_each(|(k, b)| allowed(stage, *k, b));
    let admits = r.events[before..].iter().filter(|e| matches!(e, Event::Admit(_))).count();
    assert_eq!(admits, 0, "admitted without the user's signature");
    if r.server.is_over() {
        assert!(r.send(&[1, 0, 0, 0, 1]).is_empty(), "nothing after the end");
    }
    if r.events.iter().any(|e| matches!(e, Event::Closed(_))) {
        record_checks(&r);
    }
}
