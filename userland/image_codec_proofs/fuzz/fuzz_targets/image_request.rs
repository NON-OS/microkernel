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

//! image_codec's request frame and its table of held outputs, compiled from
//! the capsule by image_codec_proofs. Neither panics. A frame parses only
//! when its header is image_codec's and its payload is exactly the length
//! declared, and the reply header written for it parses back to the same
//! request. Outputs held and let go in any order, by any clients at any
//! times, never number more than MAX_HELD, and each is let go at most once
//! and only after it was held: no output's frames are freed twice or lost.

#![no_main]

use image_codec_proofs::outputs::{Output, Outputs, MAX_HELD};
use image_codec_proofs::protocol::{parse, response_header, HDR_LEN, MAGIC, VERSION};
use libfuzzer_sys::fuzz_target;

fn frame(buf: &[u8]) {
    let Ok((req, payload)) = parse(buf) else {
        return;
    };
    assert_eq!(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]), MAGIC);
    assert_eq!(u16::from_le_bytes([buf[4], buf[5]]), VERSION);
    let declared = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as usize;
    assert_eq!(payload.len(), declared);
    assert_eq!(payload, &buf[HDR_LEN..]);
    let mut out = [0u8; HDR_LEN];
    response_header(&mut out, &req, 0);
    let Ok((back, empty)) = parse(&out) else {
        panic!("a reply header that does not parse");
    };
    assert!(empty.is_empty());
    assert_eq!((back.op, back.flags, back.request_id), (req.op, req.flags, req.request_id));
}

fn outputs(steps: &[u8]) {
    let mut table = Outputs::new();
    // Per output made, its base being its index: whether it was let go.
    let mut gone: Vec<bool> = Vec::new();
    let mut now = 0i64;
    let release = |out: Output, gone: &mut Vec<bool>| {
        assert!(out.base < gone.len(), "an output let go that was never held");
        assert!(!gone[out.base], "an output let go twice");
        gone[out.base] = true;
    };
    // Each step: an action byte, a client byte and a clock step byte.
    for step in steps.chunks_exact(3) {
        let (act, client, tick) = (step[0], u32::from(step[1] % 8), i64::from(step[2]) * 500);
        now += tick;
        let dead = act >> 4;
        let alive = |c: u32| c != u32::from(dead);
        let mut freed = Vec::new();
        table.sweep(client, now, alive, |o| freed.push(o));
        freed.into_iter().for_each(|o| release(o, &mut gone));
        if act & 1 == 0 {
            let out = Output { client, base: gone.len(), len: 4096, made_ms: now };
            gone.push(false);
            if let Some(o) = table.hold(out) {
                release(o, &mut gone);
            }
        }
        let kept = gone.iter().filter(|g| !**g).count();
        assert!(kept <= MAX_HELD, "{kept} outputs held, past {MAX_HELD}");
    }
}

fuzz_target!(|data: &[u8]| {
    let Some((&pick, body)) = data.split_first() else {
        return;
    };
    match pick % 2 {
        0 => frame(body),
        _ => outputs(body),
    }
});
