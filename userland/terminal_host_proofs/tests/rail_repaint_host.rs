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

//! A rail sample redraws the Terminal's window only while the rail or a new
//! tab's banner is on screen (term/terminal/rail_repaint.rs): with both gone,
//! the once-a-second sample no longer has the whole window composed again,
//! and the banner's uptime still ticks while it is shown, with only its row
//! presented (term/terminal/uptime_row.rs).

#[path = "../../capsule_terminal/src/term"]
mod term {
    pub mod prefs {
        #[path = "rail_flags.rs"]
        mod rail_flags;
        pub use rail_flags::RAIL_VISIBLE;
    }
    pub mod terminal {
        #[path = "rail_repaint.rs"]
        pub mod rail_repaint;
        #[path = "uptime_row.rs"]
        pub mod uptime_row;
    }
}

/// The toolkit's rectangle, as the Terminal's layout names it.
mod layout {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Rect {
        pub x: u32,
        pub y: u32,
        pub w: u32,
        pub h: u32,
    }
}

use layout::Rect;
use term::prefs::RAIL_VISIBLE;
use term::terminal::rail_repaint::{sample_repaints, uptime_row_only};
use term::terminal::uptime_row::{row_rect, uptime_damage};

const ON_TICK: &str = include_str!("../../capsule_terminal/src/term/terminal/app_impl_on_tick.rs");

#[test]
fn a_sample_redraws_only_while_the_rail_or_the_banner_is_on_screen() {
    for rails in 0..=u8::MAX {
        let shown = rails & RAIL_VISIBLE != 0;
        assert_eq!(sample_repaints(true, rails, false), shown, "rails {rails:#010b}");
        assert!(sample_repaints(true, rails, true), "the banner's uptime, rails {rails:#010b}");
        for banner in [false, true] {
            assert!(!sample_repaints(false, rails, banner), "no sample, rails {rails:#010b}");
        }
    }
}

#[test]
fn the_tick_asks_the_rule_with_the_tab_it_draws_and_keeps_sampling() {
    let banner = ON_TICK.find("let banner = self.cur().fresh;").expect("the banner read");
    let tick = ON_TICK.find("let tick = self.rail.tick();").expect("one sample a tick");
    let rule = ON_TICK
        .find("super::rail_repaint::sample_repaints(tick, self.prefs.rails, banner);")
        .expect("the rule asked");
    assert!(banner < rule && tick < rule);
    assert_eq!(ON_TICK.matches("self.rail.tick()").count(), 1, "sampled every tick, shown or not");
    assert!(ON_TICK.contains("let other = jobs::pump(self.cur()) || handed || said;"));
    assert!(ON_TICK.contains("\n        sampled || other\n    }\n"), "a sample still repaints");
}

/// A tick changes the uptime row alone only while the banner is on screen
/// and the rail, whose chart a sample also changes, is not.
#[test]
fn only_the_uptime_row_changes_when_the_banner_is_all_a_sample_redraws() {
    for rails in 0..=u8::MAX {
        let hidden = rails & RAIL_VISIBLE == 0;
        assert_eq!(uptime_row_only(true, rails, true), hidden, "rails {rails:#010b}");
        assert!(!uptime_row_only(true, rails, false), "no banner, rails {rails:#010b}");
        for banner in [false, true] {
            assert!(!uptime_row_only(false, rails, banner), "no sample, rails {rails:#010b}");
            // Whatever is row-only is a repaint the old rule asked for too.
            if uptime_row_only(true, rails, banner) {
                assert!(sample_repaints(true, rails, banner));
            }
        }
    }
}

/// The row is taken only when the tick changed nothing else and nothing a
/// frame has not drawn whole is owed, and only where the splash drew it.
#[test]
fn the_row_is_the_damage_only_when_nothing_else_is_owed() {
    let row = Rect { x: 24, y: 260, w: 900, h: 20 };
    assert_eq!(uptime_damage(true, false, Some(row)), Some(row));
    assert_eq!(uptime_damage(true, true, Some(row)), None, "a change still owed");
    assert_eq!(uptime_damage(false, false, Some(row)), None, "more than the row changed");
    assert_eq!(uptime_damage(true, false, None), None, "the splash was never drawn");
    let row_only = ON_TICK.find("let row = row && !other && self.frame_key() == before;");
    let before = ON_TICK.find("let before = self.frame_key();");
    assert!(before.is_some() && before < row_only, "the frame's key read before the tick");
    assert!(ON_TICK.contains("self.whole_owed |= (sampled || other) && !row;"));
    let event = include_str!("../../capsule_terminal/src/term/terminal/app_impl.rs");
    assert!(event.contains("self.uptime_only = false;"), "a key or click takes the row's place");
    assert!(
        event.contains("self.whole_owed |= outcome == EventOutcome::Repaint && !self.line_only;")
    );
    let paint = include_str!("../../capsule_terminal/src/term/terminal/app_impl_paint.rs");
    assert!(paint.contains(
        "if !self.partial {\n            (self.whole_owed, self.uptime_only) = (false, false);"
    ));
    let damage = include_str!("../../capsule_terminal/src/term/terminal/damage.rs");
    assert!(damage.contains("uptime_row::uptime_damage(uptime, self.whole_owed, row)"));
    assert!(paint.contains("let whole_frame = !self.partial;"), "drawn whole, other buffer owed");
}

/// The row runs from the facts' left edge to the panel's right, a row high,
/// and is never made from a place off the buffer or of no width.
#[test]
fn the_uptime_row_is_the_facts_band() {
    assert_eq!(row_rect(24, 260, 924, 20), Some(Rect { x: 24, y: 260, w: 900, h: 20 }));
    assert_eq!(row_rect(24, 260, 24, 20), None);
    assert_eq!(row_rect(24, 260, 10, 20), None);
    assert_eq!(row_rect(-1, 260, 900, 20), None);
    assert_eq!(row_rect(24, -5, 900, 20), None);
    assert_eq!(row_rect(i32::MIN, 0, i32::MAX, 20), None, "no overflow on the width");
    let fetch = include_str!("../../capsule_terminal/src/paint/fetch.rs");
    let drawn = fetch.find("row(fb, ix, y, \"uptime\"").expect("the uptime row");
    let noted = fetch.find(
        "uptime_at.set(crate::term::terminal::uptime_row::row_rect(ix, y, edge, ROW as u32));",
    );
    assert!(noted.is_some_and(|n| n > drawn), "noted where it was just drawn");
}

/// The banner is what a fresh tab draws, so `fresh` is the banner on screen.
#[test]
fn a_fresh_tab_is_the_one_that_draws_the_banner() {
    let tab = include_str!("../../capsule_terminal/src/paint/compose_tab.rs");
    assert!(tab.contains("} else if state.fresh {\n        draw_fetch("));
}
