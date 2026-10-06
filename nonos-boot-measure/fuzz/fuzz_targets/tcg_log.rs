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

//! The TCG log reader on any bytes: no panic; the loader's growing reads of a
//! prefix agree with the kernel's reads of the whole log; a replayed log names
//! no more events than its cap.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_boot_measure::tcg::{
    event, event_walk, grow, replay, spec_id, spec_id_walk, Walk, MAX_EVENTS, MAX_EVENT_BYTES,
};

fuzz_target!(|log: &[u8]| {
    if let Ok(r) = replay(log) {
        assert!(r.events <= MAX_EVENTS);
    }
    let whole = spec_id(log);
    let grown = grow(32, log.len(), |n| log.get(..n), spec_id_walk);
    assert_eq!(whole.as_ref().map(|h| h.1).ok(), grown.as_ref().map(|h| h.1).ok());
    let Ok((banks, at)) = whole else { return };
    let rest = &log[at..];
    let once = event(&banks, rest).map(|e| e.len);
    let walked = grow(
        12,
        MAX_EVENT_BYTES + 1024,
        |n| rest.get(..n),
        |p| {
            event_walk(&banks, p).map(|w| match w {
                Walk::Done(e) => Walk::Done(e.len),
                Walk::Needs(n) => Walk::Needs(n),
            })
        },
    );
    if let Ok(n) = once {
        assert_eq!(walked, Ok(n));
    }
});
