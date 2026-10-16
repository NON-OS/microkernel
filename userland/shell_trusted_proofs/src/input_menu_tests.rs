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

//! A display's input is switched only by a choice made with the router's
//! input on an open menu, of a row the display does not already show, and
//! once per opening. Seeded sequences of opens, keys and presses are played
//! against the test's own model of that rule.

use crate::input_menu_rig::{by, next, offer, rows, Model};
use crate::state::input_menu::{InputMenu, KEY_DOWN, KEY_UP};
use crate::state::input_offers::Offer;

#[test]
fn a_switch_is_made_only_by_a_routed_choice_of_a_row_not_shown() {
    let (mut s, mut m, mut w) = (0x1A9B_C0DE_u64, InputMenu::default(), Model::default());
    let mut seen = [0usize; 4];
    for _ in 0..200_000 {
        next(&mut s);
        let pick = match s % 6 {
            0 => {
                let offers: Vec<Offer> =
                    (0..(s >> 8) as u32 % 4).map(|i| offer(s >> 12, i)).collect();
                w = Model { open: m.open(&offers), at: 0, rows: rows(&offers) };
                assert_eq!(w.open, !w.rows.is_empty());
                seen[0] += w.open as usize;
                None
            }
            1 | 2 => {
                let up = s % 6 == 1;
                assert!(m.key(by(), if up { KEY_UP } else { KEY_DOWN }).is_none());
                let last = w.rows.len().saturating_sub(1);
                w.at = match (w.open, up) {
                    (false, _) => w.at,
                    (true, true) => w.at.saturating_sub(1),
                    (true, false) => (w.at + 1).min(last),
                };
                None
            }
            3 => w.open.then_some((m.key(by(), 0x0D), Some(w.at))),
            4 => {
                let row = Some((s >> 40) as usize % 30).filter(|_| s >> 39 & 1 == 0);
                Some((m.choose(by(), row), row))
            }
            _ => {
                let esc = s >> 20 & 1 == 0;
                let got = m.key(by(), if esc { 0x1B } else { b'x' as u32 });
                assert!(got.is_none(), "Esc and other keys never switch");
                w.open &= !esc;
                seen[3] += 1;
                None
            }
        };
        if let Some((got, row)) = pick {
            let want = row.and_then(|r| w.rows.get(r)).filter(|r| w.open && !r.2);
            assert_eq!(got.map(|g| (g.identity(), g.input())), want.map(|r| (r.0, r.1)));
            seen[1 + want.is_some() as usize] += 1;
            w.open = false;
        }
        assert_eq!((m.showing(), m.at()), (w.open, w.at), "after {s:x}");
    }
    assert!(seen.iter().all(|&k| k >= 5_000), "{seen:?}");
}
