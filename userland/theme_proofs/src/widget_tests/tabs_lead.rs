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

//! Tabs: a lead is an `ICON` square inside its own tab with the label
//! `SPACE.sm` past it, and a busy dot lands in a busy tab before its label.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::{ICON, SPACE};
use nonos_toolkit::widgets::Tabs;

use super::boxes::{boxes, inside, within};
use super::canvas::Canvas;
use super::hide::Hide;
use super::mask::Masked;
use super::row_marks::Marks;
use super::serial::{serial, SCALES};
use super::tabs::{BUSY, ICONS, LABELS, LONGS};

#[test]
fn tabs_lead_sits_in_its_tab_before_the_label() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let (side, gap) = (scale().px(ICON), scale().px(u32::from(SPACE.sm)));
        let t = Tabs::new(&LABELS, 0).icons(&ICONS);
        let mut m = Marks::default();
        t.draw(&mut m, boxes(t.size())[0]);
        for (i, sq) in m.1.iter().enumerate() {
            let body = t.tab(boxes(t.size())[0], i).unwrap();
            let after = sq.x + side + gap;
            if sq.w != side || !within(*sq, body) || m.0[i].0 != after as i32 {
                fails.push(format!("tab {i} at {q}/4: {sq:?} in {body:?}, label {:?}", m.0[i]));
            }
        }
        let busy = t.busy(&BUSY);
        for at in boxes(busy.size()).into_iter().chain(boxes(Tabs::new(&LONGS, 0).size())) {
            let mut m = Marks::default();
            busy.draw(&mut m, at);
            let lit = Canvas::new(at, 0).painted(|fb| busy.draw(fb, at));
            let off = Canvas::new(at, 0)
                .painted(|fb| busy.draw(&mut Masked { fb, hide: Hide::Dots, rounds: 0 }, at));
            let lead = |i: usize, p: (u32, u32)| {
                let before = m.0.get(i).is_none_or(|l| (p.0 as i32) < l.0);
                busy.tab(at, i).is_some_and(|b| inside(b, p)) && before
            };
            if let Some(p) = lit.diff(&off).into_iter().find(|&p| !lead(1, p) && !lead(2, p)) {
                fails.push(format!("busy dot outside its lead in {at:?} at {q}/4: {p:?}"));
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
