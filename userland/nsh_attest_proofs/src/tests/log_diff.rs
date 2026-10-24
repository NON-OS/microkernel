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

//! The kernel replays PCR 4 from the same log for its own loader check
//! (nonos-boot-measure). Its replay, compiled from its source, and this
//! one give the same PCR 4 and the same last application.

#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../../../nonos-boot-measure/src/tcg/mod.rs"]
mod kernel_tcg;

use nsh_attest::log::replay;

use crate::build::tcg::{EV_ACTION, EV_APP};
use crate::build::tcg_event::digest;
use crate::build::tcg_log::{loader, Log};

fn agree(log: &[u8]) {
    let mine = replay(log).expect("this replay reads it");
    let theirs = kernel_tcg::replay(log).expect("the kernel's reads it");
    assert_eq!(mine.pcrs[4], theirs.pcr4);
    assert_eq!(mine.last_application, theirs.last_application);
    assert_eq!(mine.events, theirs.events);
}

#[test]
fn the_kernel_and_the_client_replay_pcr_4_alike() {
    agree(&Log::standard().bytes);
    agree(&Log::empty().bytes);
    let mut l = Log::standard();
    l.ext(4, EV_APP, digest(b"a shell")).note(4, EV_ACTION, b"returning");
    agree(&l.bytes);
    let mut l = Log::empty();
    for i in 0..200u32 {
        match i % 3 {
            0 => l.ext(i % 8, EV_APP, digest(&i.to_be_bytes())),
            _ => l.note(i % 8, EV_ACTION, &i.to_be_bytes()),
        };
    }
    l.ext(4, EV_APP, loader());
    agree(&l.bytes);
}
