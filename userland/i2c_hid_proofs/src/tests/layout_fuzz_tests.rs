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

//! Three hundred thousand hand-built layouts, every field anywhere: the
//! decode never reads past a report's body, whatever offsets and sizes the
//! walk's own bounds would never let through, and a position stays inside
//! its range. The checks the walk makes unreachable are proved here.

use super::rng::Rng;
use crate::hid::{decode_mouse, decode_touch, Field, MouseLayout, TouchLayout};

fn field(rng: &mut Rng) -> Field {
    let pick = |rng: &mut Rng| match rng.below(5) {
        0 => u32::MAX - rng.below(64) as u32,
        1 => rng.below(64) as u32,
        2 => rng.below(4096) as u32,
        3 => 0,
        _ => rng.word() as u32,
    };
    let (bit_offset, bit_size) = (pick(rng), pick(rng));
    let bound = |rng: &mut Rng| match rng.below(3) {
        0 => i32::MIN,
        1 => i32::MAX,
        _ => rng.word() as i32,
    };
    Field { bit_offset, bit_size, logical_min: bound(rng), logical_max: bound(rng) }
}

#[test]
fn hand_built_layouts_with_any_field_decode_only_inside_the_body() {
    let mut rng = Rng::seeded(0xD14_0001);
    for round in 0..300_000u32 {
        let mut t = TouchLayout { report_id: rng.byte(), ..TouchLayout::default() };
        [t.x, t.y, t.tip, t.contact_count, t.contact_id, t.button, t.confidence] =
            core::array::from_fn(|_| field(&mut rng));
        let (buttons, button_count) = (field(&mut rng), rng.byte());
        let (mx, my, wheel) = (field(&mut rng), field(&mut rng), field(&mut rng));
        let m = MouseLayout { report_id: rng.byte(), buttons, button_count, x: mx, y: my, wheel };
        let len = rng.below(300);
        let mut frame = rng.bytes(len);
        if let (Some(b), true) = (frame.first_mut(), rng.one_in(2)) {
            *b = t.report_id;
        }
        if let Some(s) = decode_touch(&frame, &t) {
            let body = 8 * (frame.len() - usize::from(t.report_id != 0)) as u64;
            for f in [t.x, t.y, t.tip, t.contact_count, t.contact_id, t.button, t.confidence] {
                let end = u64::from(f.bit_offset) + u64::from(f.bit_size);
                assert!(f.bit_size == 0 || end <= body, "round {round}: read past the body");
            }
            assert!(s.x <= s.x_max.max(0) as u32 && s.y <= s.y_max.max(0) as u32, "round {round}");
        }
        let _ = decode_mouse(&frame, &m);
    }
}
