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

use capsule_browser_proofs::browser::settings::look::Shows;
use capsule_browser_proofs::browser::shell::kept::MAX_SIZE;
use capsule_browser_proofs::browser::ui::settings_geom::Control;
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, geom, live, mid, paint, W};

const STEPS: [Control; 2] = [Control::Smaller, Control::Larger];

#[test]
fn the_text_size_steps_are_disabled_at_the_ends() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let g = geom();
        for (size, dead) in [(0, Control::Smaller), (MAX_SIZE, Control::Larger)] {
            let s = Shows { size, ..live() };
            let mut c = Controls::acting_on_press();
            let unlit = paint(&g, &s, &mut c);
            for step in STEPS {
                let (x, y) = mid(g.rect(step).unwrap());
                c.moved(x, y);
                let act = if step == dead { Step::Idle } else { Step::Act(step) };
                assert_eq!(c.pressed(x, y), act, "{step:?} at size {size}, {q}");
                assert_eq!(paint(&g, &s, &mut c) == unlit, step == dead, "{step:?} at {size}, {q}");
                c.released();
                c.left();
            }
        }
    }
}

#[test]
fn the_steps_are_a_minus_and_a_plus_drawn_in_their_boxes() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let (g, mut c) = (geom(), Controls::acting_on_press());
        let page = paint(&g, &live(), &mut c);
        let drawn = |ctl: Control| {
            let r = g.rect(ctl).unwrap();
            let cells = (0..r.h).flat_map(|y| (0..r.w).map(move |x| (r.x + x, r.y + y)));
            cells.map(|(x, y)| page[(y * W + x) as usize]).collect::<Vec<u32>>()
        };
        let (minus, plus) = (drawn(Control::Smaller), drawn(Control::Larger));
        let ink = |v: &[u32]| v.iter().filter(|&&p| p != fill(v) && p != 0).count();
        assert!(ink(&minus) > 0, "the minus step draws no icon at {q}");
        assert!(ink(&plus) > ink(&minus), "plus should carry more ink than minus at {q}");
    }
}

/* The colour most of a step's box is painted: its body. */
fn fill(v: &[u32]) -> u32 {
    let mut n = std::collections::HashMap::new();
    v.iter().filter(|&&p| p != 0).for_each(|&p| *n.entry(p).or_insert(0) += 1);
    n.into_iter().max_by_key(|&(_, k)| k).map_or(0, |(p, _)| p)
}
