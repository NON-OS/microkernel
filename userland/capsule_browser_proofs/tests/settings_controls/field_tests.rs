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

use capsule_browser_proofs::browser::settings::controls::area;
use capsule_browser_proofs::browser::settings::field::{caret_at, entry};
use capsule_browser_proofs::browser::shell::settings_ui::SettingsUi;
use capsule_browser_proofs::browser::ui::settings_geom::{Control, Field};
use nonos_toolkit::paint::PaintBuffer;

use super::kit::{at_scale, changed, geom, W};

pub const TEXT: &str = "https://ex.org/s?q=%s Grüße 日本";
pub const F: Field = Field::Search;

/* The page's pixels with field `F` of `ui` drawn, its caret left out unless
 * `caret`. */
pub fn draw(ui: &SettingsUi, caret: bool) -> Vec<u32> {
    let r = geom().rect(Control::Field(F)).unwrap();
    let (w, h) = (W as u32, (r.bottom() + 2) as u32);
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    let e = entry(ui, F);
    (if caret { e } else { e.caret(None) }).draw(&mut fb, area(r));
    px
}

pub fn typing(anchor: usize, caret: usize) -> SettingsUi {
    let mut ui = SettingsUi { focus: Some(F), ..SettingsUi::default() };
    ui.search.set(TEXT);
    (ui.search.anchor, ui.search.caret) = (anchor, caret);
    ui
}

/* The x the field draws its caret at, at byte `c`. */
pub fn caret_x(c: usize) -> i32 {
    let ui = typing(c, c);
    changed(&draw(&ui, false), &draw(&ui, true)).into_iter().map(|p| p.0).min().unwrap()
}

#[test]
fn a_click_in_a_field_puts_the_caret_on_the_nearest_boundary() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let r = geom().rect(Control::Field(F)).unwrap();
        let b: Vec<usize> = TEXT.char_indices().map(|(i, _)| i).chain([TEXT.len()]).collect();
        for &c in &b {
            let x = caret_x(c);
            assert!(x > r.x && x < r.right(), "caret {c} drawn outside the field at {q}");
            assert_eq!(caret_at(&typing(c, c), F, r, x), c, "{c} clicked back at {q}");
        }
        let end = typing(TEXT.len(), TEXT.len());
        let at = |x| caret_at(&end, F, r, x);
        for pair in b.windows(2).filter(|p| p.iter().all(|&c| at(caret_x(c)) == c)) {
            let (x0, half) = (caret_x(pair[0]), (caret_x(pair[1]) - caret_x(pair[0])) / 2);
            if half > 1 {
                assert_eq!(at(x0 + half - 1), pair[0], "{pair:?} short of half at {q}");
                assert_eq!(at(x0 + half + 1), pair[1], "{pair:?} past half at {q}");
            }
        }
        assert_eq!(caret_at(&SettingsUi::default(), F, r, r.x + r.w / 2), 0, "empty at {q}");
    }
}
