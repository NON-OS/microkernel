// NONOS Operating System (AGPL-3.0-or-later)
//! The Editing switches, at 1x and 2x: each draws its setting, on in the
//! accent and off without it, and a press on its row flips the setting and
//! repaints only that row.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Controls, Row, Switch};

use super::control_tests::{crop, paint, row_at};
use super::pick::{act, Pick};
use super::sect::{section, Ctl};
use super::sect_state::sect_on;
use crate::editor::square_tests::lock;
use crate::editor::strip_tests::changed;

#[test]
fn a_switch_shows_its_setting_and_a_press_on_its_row_flips_it() {
    let _g = lock();
    let rows = section(0).unwrap().rows.iter().enumerate();
    for (q, (i, spec)) in [4, 8].into_iter().flat_map(|q| rows.clone().map(move |r| (q, r))) {
        let Ctl::Toggle(bit) = spec.1 else { continue };
        (set_scale(q), super::state::select_nav(0));
        let (c, row) = (&mut Controls::acting_on_press(), row_at(0, i));
        let track = Row::new(spec.0).trailing(Switch::new(false).size().0).trailing_rect(row);
        let on = |px: &[u32]| crop(px, track).contains(&roles().accent.argb());
        for _ in 0..2 {
            let (was, before) = (sect_on(0, bit), paint(c));
            assert_eq!(on(&before), was, "row {i} draws its setting at {q}");
            assert!(act(Pick::Row(i)) && sect_on(0, bit) != was, "row {i} flips at {q}");
            let diff = changed(&before, &paint(c));
            let here = diff.iter().all(|&(x, y)| c.at(x, y) == Some(Pick::Row(i)));
            assert!(!diff.is_empty() && here, "only row {i} repaints at {q}");
        }
    }
    set_scale(4);
}
