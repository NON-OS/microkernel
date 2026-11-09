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

use capsule_browser_proofs::browser::settings::caption::caption;
use capsule_browser_proofs::browser::settings::look::Shows;
use capsule_browser_proofs::browser::ui::settings_geom::{Control, SettingsGeom};
use nonos_toolkit::widgets::{Button, Controls};

use super::kit::{at_scale, changed, controls, geom, live, paint, H, W};

/* Every pixel that moves between `a` and `b` lies on one of `on`. */
fn moves_only(a: &Shows, b: &Shows, on: &[Control], q: u32) {
    let g = geom();
    let mut c = Controls::acting_on_press();
    let before = paint(&g, a, &mut c);
    let diff = changed(&before, &paint(&g, b, &mut c));
    assert!(!diff.is_empty(), "{on:?} at {q}");
    assert!(diff.iter().all(|&(x, y)| on.contains(&c.at(x, y).unwrap())), "{on:?} at {q}");
}

#[test]
fn a_new_choice_moves_only_between_its_rows_buttons() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let s = live();
        let theme = [Control::Appearance(0), Control::Appearance(1)];
        moves_only(&s, &Shows { appearance: 1, ..s }, &theme, q);
        let net = [Control::Network(0), Control::Network(2)];
        moves_only(&s, &Shows { network: Some(2), ..s }, &net, q);
        let search = [Control::Search(0), Control::Search(1)];
        moves_only(&s, &Shows { custom: true, ..s }, &search, q);
        let width = [Control::ReaderWidth(0), Control::ReaderWidth(1)];
        moves_only(&s, &Shows { width: 1, ..s }, &width, q);
    }
}

#[test]
fn remove_all_bookmarks_shows_its_second_click_in_its_own_box() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let s = live();
        let armed = Shows { armed: true, ..s };
        moves_only(&s, &armed, &[Control::RemoveBookmarks], q);
        assert_eq!(caption(&armed, Control::RemoveBookmarks), "Click again to remove 2 bookmarks");
        assert_eq!(caption(&s, Control::RemoveBookmarks), "Remove all bookmarks");
    }
}

#[test]
fn every_choice_caption_fits_its_button() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let s = live();
        for (ctl, r) in controls(&SettingsGeom::new(W * q as i32 / 4, H, 0)) {
            let words = caption(&s, ctl);
            let fits = Button::new(&words).size().0 <= r.w as u32;
            assert!(words.is_empty() || fits, "{ctl:?} {words} {r:?} at {q}");
        }
    }
}
