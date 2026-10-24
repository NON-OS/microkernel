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

//! The session bodies: the first byte picks one, the rest is its body.
//! What parses encodes back to the very bytes where an encoder exists.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_wire::{abort, data, signal, window, AuthOk, EscalateResult, Reason, RecordPart};

fuzz_target!(|input: &[u8]| {
    let Some((pick, body)) = input.split_first() else { return };
    let mut out = [0u8; 2048];
    match pick % 7 {
        0 => {
            if let Ok(w) = window(body) {
                assert_eq!(w.encode(&mut out).map(|n| &out[..n]), Ok(body));
            }
        }
        1 => {
            if let Ok(ok) = AuthOk::parse(body) {
                let caps: Vec<&[u8]> = ok.caps.iter().collect();
                let n = AuthOk::encode(ok.session_id, ok.idle_s, &caps, &mut out).unwrap();
                assert_eq!(&out[..n], body);
            }
        }
        2 => {
            if let Ok(r) = EscalateResult::parse(body) {
                assert_eq!(r.encode(&mut out).map(|n| &out[..n]), Ok(body));
            }
        }
        3 => {
            if let Ok(r) = abort(body) {
                assert!(r != Reason::Ok && r.ends_session());
            }
        }
        4 => {
            if let Ok(p) = RecordPart::parse(body) {
                assert!(!p.bytes.is_empty() && p.bytes.len() < nsh_wire::MAX_DATA);
            }
        }
        5 => {
            let _ = signal(body);
        }
        _ => {
            if let Ok(d) = data(body) {
                assert!(!d.is_empty() && d.len() <= nsh_wire::MAX_DATA);
            }
        }
    }
});
