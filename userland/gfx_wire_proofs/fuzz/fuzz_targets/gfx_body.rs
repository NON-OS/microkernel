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

//! Every NVGP body, compiled from nonos_gfx_wire by gfx_wire_proofs. None
//! panics. Whatever a body decodes to encodes and decodes back to itself; a
//! flush or a mode list holds no more entries than the body carries or the
//! wire allows; a cursor is no larger than 64 a side with its hot spot
//! inside it; the resize rule over any CAPS and MODES asks for a size in
//! the adapter's range, or else a listed mode no larger than wanted, and
//! never the size already shown.

#![no_main]

use gfx_wire_proofs::body::caps::{Caps, CAPS_LEN};
use gfx_wire_proofs::body::caps_flags::MODE_RANGE;
use gfx_wire_proofs::body::cursor::{Cursor, MAX_SIDE};
use gfx_wire_proofs::body::flush::{Flush, MAX_RECTS};
use gfx_wire_proofs::body::mode::{MAX_MODES, MODE_LEN};
use gfx_wire_proofs::body::modes::Modes;
use gfx_wire_proofs::body::scanout::Scanout;
use gfx_wire_proofs::body::state::State;
use gfx_wire_proofs::body::surface::Surface;
use gfx_wire_proofs::bytes::u32_at;
use gfx_wire_proofs::fit::fit;
use libfuzzer_sys::fuzz_target;
use nonos_modes::fit::MIN;

/// A fixed-size body: decoded, encoded, decoded again to the same.
macro_rules! round {
    ($t:ty, $b:expr) => {
        if let Ok(v) = <$t>::decode($b) {
            assert_eq!(<$t>::decode(&v.encode()), Ok(v), "a body reads back to another");
        }
    };
}

fn flush(b: &[u8]) {
    let Ok(f) = Flush::decode(b) else {
        return;
    };
    assert!((1..=MAX_RECTS).contains(&f.len) && b.len() >= 8 + f.len * 16);
    let mut out = [0u8; 8 + MAX_RECTS * 16];
    let n = f.encode(&mut out).ok();
    assert_eq!(n, Some(8 + f.len * 16));
    assert_eq!(Flush::decode(&out), Ok(f));
}

fn modes(b: &[u8]) {
    let Ok(m) = Modes::decode(b) else {
        return;
    };
    assert!(m.len <= MAX_MODES && b.len() >= 4 + m.len * MODE_LEN);
    let mut out = [0u8; 4 + MAX_MODES * MODE_LEN];
    assert_eq!(m.encode(&mut out).ok(), Some(4 + m.len * MODE_LEN));
    assert_eq!(Modes::decode(&out), Ok(m));
}

fn cursor(b: &[u8]) {
    let Ok(c) = Cursor::decode(b) else {
        return;
    };
    assert!(c.width <= MAX_SIDE && c.height <= MAX_SIDE);
    assert!(c.hot_x < c.width.max(1) && c.hot_y < c.height.max(1));
    assert_eq!(Cursor::decode(&c.encode()), Ok(c));
}

fn resize(b: &[u8]) {
    // Four words of want and current, a CAPS, then a MODES.
    let w = |at| u32_at(b, at).unwrap_or(0);
    let (want, current) = ((w(0), w(4)), (w(8), w(12)));
    let Some(rest) = b.get(16..) else {
        return;
    };
    let caps = Caps::decode(rest).unwrap_or_default();
    let list = rest.get(CAPS_LEN..).and_then(|m| Modes::decode(m).ok()).unwrap_or_default();
    if let Some(size) = fit(want, current, &caps, list.as_slice()) {
        assert_ne!(size, current, "the size already shown asked for");
        if caps.has(MODE_RANGE) {
            assert!(size.0 >= MIN.0 && size.1 >= MIN.1, "a size under the smallest asked for");
            assert!(size.0 <= caps.max_width && size.1 <= caps.max_height, "a size past the range");
        } else {
            assert!(size.0 <= want.0 && size.1 <= want.1, "a mode larger than wanted");
            assert!(list.as_slice().iter().any(|m| (m.width, m.height) == size), "a mode not listed");
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let Some((&pick, body)) = data.split_first() else {
        return;
    };
    match pick % 8 {
        0 => round!(Caps, body),
        1 => round!(Scanout, body),
        2 => round!(State, body),
        3 => round!(Surface, body),
        4 => flush(body),
        5 => modes(body),
        6 => cursor(body),
        _ => resize(body),
    }
});
