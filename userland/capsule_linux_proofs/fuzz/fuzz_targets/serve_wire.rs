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

//! What SERVE_ONE's capsule hands the personality (P.9,
//! the SERVE_ONE design). Any bytes as one frame: it decodes only when
//! `len` names exactly the bytes after its header, MAX_BODY at most, and a
//! question only with status 0. The same bytes as steps against the wire's
//! server as it ships (serve_steps.rs).

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_serve_wire::{decode, request, HEADER, MAGIC, MAX_BODY};

mod serve_steps;

fuzz_target!(|b: &[u8]| {
    if let Ok((h, body)) = decode(b) {
        assert_eq!(h.len as usize, body.len(), "a body not the one len names");
        assert_eq!(HEADER + body.len(), b.len(), "bytes past the frame");
        assert!(body.len() <= MAX_BODY, "a body past MAX_BODY");
        assert!(request(b).is_err() || h.status == 0, "a question carrying a status");
    } else {
        assert!(request(b).is_err(), "a question no frame");
    }
    serve_steps::run(b);
});

/// An answer: the magic, then exactly the bytes its `len` names.
pub fn whole(f: &[u8]) {
    assert!(f.len() >= HEADER && f[..4] == MAGIC, "an answer short or unmarked");
    assert_eq!(u32::from_le_bytes([f[8], f[9], f[10], f[11]]) as usize, f.len() - HEADER);
}
