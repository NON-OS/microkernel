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

//! At 1x, 1.25x, 1.5x and 2x, in a window grown by the same scale, a card's
//! action and every source tab answer the pointer where they are drawn. Its
//! own binary, so setting the scale races no other test.

use nonos_market_proofs::store::listing::Listing;
use nonos_market_proofs::store::state::State;
use nonos_market_proofs::store::tab::TABS;
use nonos_market_proofs::store::ui::geometry::{action_rect, list_w, row_top};
use nonos_market_proofs::store::ui::hover::target_at;
use nonos_market_proofs::store::ui::metrics::{metrics, WIN_W};
use nonos_market_proofs::store::ui::tab_strip::{band, tab_at};
use nonos_market_proofs::store::ui::target::Target;
use nonos_toolkit::design::scale::{scale, set_scale};

fn mid(at: u32, len: u32) -> i32 {
    (at + len / 2) as i32
}

#[test]
fn the_action_and_every_tab_answer_where_they_are_drawn_at_each_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, pad_x) = (scale().px(WIN_W), metrics().pad_x);
        let one = Listing::new(b"nonos.a".to_vec(), [0; 32], b"a".to_vec(), true);
        let state = State { listings: vec![one], scroll: 0, rows: 1, fb_w: w, hover: Target::None };
        let (ax, ay, aw, ah) = action_rect(pad_x, row_top(0), list_w(w));
        let at = target_at(&state, mid(ax, aw), mid(ay, ah));
        assert!(at == Target::Action(0), "q={q}: the action does not answer");
        let strip = band(w);
        let y = mid(strip.y, strip.h);
        let lit: Vec<_> = (strip.x..strip.x + strip.w)
            .filter_map(|x| tab_at(w, x as i32, y).map(|i| (x as i32, i)))
            .collect();
        for i in 0..TABS.len() {
            let x = lit.iter().find(|&&(_, t)| t == i).map(|&(x, _)| x);
            let x = x.unwrap_or_else(|| panic!("q={q}: tab {i} is off the strip"));
            assert!(target_at(&state, x, y) == Target::Tab(i), "q={q}: tab {i}");
        }
    }
    set_scale(4);
}
