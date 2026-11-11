// NONOS Operating System (AGPL-3.0-or-later)
//! At 1x, 1.25x, 1.5x and 2x a click lands on what was drawn under it: each
//! sidebar entry's painted box and each search result's row, both read from
//! the metrics at the scale. Its own binary, so setting the scale races no
//! other test.

use capsule_settings_proofs::settings::section::SECTIONS;
use capsule_settings_proofs::settings::ui::metrics::metrics;
use capsule_settings_proofs::settings::ui::nav_geom::{at, row_w, row_x, row_y};
use capsule_settings_proofs::settings::ui::results_geom::{index_at, row_y as result_y};
use nonos_toolkit::design::scale::set_scale;

#[test]
fn a_nav_or_result_click_lands_on_what_was_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let m = metrics();
        for (i, s) in SECTIONS.iter().enumerate() {
            let (x, y, w, h) = (row_x() as i32, row_y(i) as i32, row_w() as i32, m.nav_h as i32);
            for (px, py) in [(x, y), (x + w - 1, y + h - 1)] {
                assert!(at(px, py) == Some(*s), "q={q}: nav entry {i} misses its box");
            }
            assert!(at(x, y + h).is_none() || m.nav_gap == 0, "q={q}: the gap clicks");
        }
        for i in 0..12 {
            let (top, bottom) = (result_y(i) as i32, (result_y(i) + m.row_h) as i32);
            assert_eq!(index_at(top, 0, 12), Some(i), "q={q}: result {i}'s top");
            assert_eq!(index_at(bottom - 1, 0, 12), Some(i), "q={q}: result {i}'s bottom");
            assert_ne!(index_at(bottom, 0, 12), Some(i), "q={q}: result {i} overruns");
        }
    }
    set_scale(4);
}
