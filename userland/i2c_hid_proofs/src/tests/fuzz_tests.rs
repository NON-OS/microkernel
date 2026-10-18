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

//! Two hundred thousand hostile report descriptors through both walks, and
//! hostile frames through every layout they give. Nothing may panic, with
//! overflow checks on as these proofs build; every field lies inside one
//! report; a decoded position stays inside its logical range.

use super::hostile_gen::hostile_descriptor;
use super::rng::Rng;
use crate::hid::{decode_mouse, decode_touch, parse_mouse_layout, parse_report_descriptor};
use crate::hid::{MouseLayout, TouchLayout};

const ROUNDS: u32 = 200_000;
/// The bits of the longest report a layout may place a field in: Linux's
/// 16 KiB report buffer (parse/limits.rs).
const REPORT_BITS: u64 = 16384 * 8;

fn fields(t: &TouchLayout, m: &MouseLayout) -> [(u32, u32); 14] {
    let f = [t.x, t.y, t.tip, t.contact_count, t.contact_id, t.button, t.confidence];
    let g = [t.input_mode, t.surface_switch, t.button_switch, m.x, m.y, m.wheel, m.buttons];
    let mut out = [(0, 0); 14];
    for (o, f) in out.iter_mut().zip(f.iter().chain(g.iter())) {
        *o = (f.bit_offset, f.bit_size);
    }
    out
}

#[test]
fn hostile_descriptors_lay_out_only_what_fits_and_decode_without_panic() {
    let mut rng = Rng::seeded(0x5eed_0014);
    for round in 0..ROUNDS {
        let desc = hostile_descriptor(&mut rng);
        let (t, m) = (parse_report_descriptor(&desc), parse_mouse_layout(&desc));
        for (at, size) in fields(&t, &m) {
            assert!(size <= 32, "round {round}: a {size}-bit field: {desc:02x?}");
            let end = u64::from(at) + u64::from(size);
            assert!(end <= REPORT_BITS, "round {round}: a field ends at {end}: {desc:02x?}");
        }
        for _ in 0..4 {
            let len = rng.below(80);
            let mut frame = rng.bytes(len);
            if let (Some(b), true) = (frame.first_mut(), rng.one_in(2)) {
                *b = t.report_id;
            }
            if let Some(s) = decode_touch(&frame, &t) {
                assert!(s.x <= s.x_max.max(0) as u32, "round {round}: x past its range");
                assert!(s.y <= s.y_max.max(0) as u32, "round {round}: y past its range");
            }
            let _ = decode_mouse(&frame, &m);
        }
    }
}
