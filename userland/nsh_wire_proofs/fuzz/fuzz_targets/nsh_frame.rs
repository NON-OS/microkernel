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

//! Frame lengths and order: at every stage, a length taken is within the
//! stage's cap, and a frame split is one the stage admits, with its body
//! the rest of the input.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_wire::{split, wire_len, Expect, TAG_LEN};

const ALL: [Expect; 7] = [
    Expect::ClientHello,
    Expect::ServerHello,
    Expect::HostProof,
    Expect::UserAuth,
    Expect::AuthResult,
    Expect::ServerSession,
    Expect::ClientSession,
];

fuzz_target!(|data: &[u8]| {
    for e in ALL {
        if let Some(len) = data.get(..4) {
            if let Ok(n) = wire_len(len.try_into().unwrap(), e) {
                let tag = if e.sealed() { TAG_LEN } else { 0 };
                assert!(n > tag && n <= e.cap() + tag);
            }
        }
        if let Ok((kind, body)) = split(data, e) {
            assert!(e.admits(kind));
            assert_eq!(body.len() + 1, data.len());
            assert!(data.len() <= e.cap());
        }
    }
});
