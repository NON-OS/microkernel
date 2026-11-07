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

//! A clock tick or a key in the field while locked shows the band from the
//! time to the count down, not the whole screen.

use crate::shell_lock::geometry::{places, tick_band, tick_bounds};

const CLOCK: &str =
    include_str!("../../../../capsule_desktop_shell/src/server/runner/refresh_clock.rs");
const TICK: &str = include_str!("../../../../capsule_desktop_shell/src/server/lock/tick.rs");
const BAND: &str = include_str!("../../../../capsule_desktop_shell/src/server/lock/band.rs");
const CHECK: &str = include_str!("../../../../capsule_desktop_shell/src/server/lock/check.rs");
const NOTE_H: u32 = 4 * 22;

fn holds(outer: (u32, u32, u32, u32), inner: (u32, u32, u32, u32)) -> bool {
    inner.0 >= outer.0
        && inner.1 >= outer.1
        && inner.0 + inner.2 <= outer.0 + outer.2
        && inner.1 + inner.3 <= outer.1 + outer.3
}

#[test]
fn one_output_band_holds_the_time_the_date_the_field_and_the_note() {
    let o = (0, 0, 1920, 1080);
    let p = places(o, 1);
    let b = tick_band(o, 1, NOTE_H);
    assert_eq!(b, (0, p.time_y, 1920, p.note_y + NOTE_H - p.time_y));
    assert!(holds(b, p.field), "the field's count of dots is in it");
    assert!(b.1 <= p.date_y && b.1 + b.3 > p.note_y);
    assert!(b.3 < 1080 / 2, "a band, not the screen: {b:?}");
}

#[test]
fn a_band_never_leaves_its_output() {
    for u in [1, 2, 3] {
        for o in [(0, 0, 1920, 1080), (1920, 0, 1280, 400), (0, 0, 320, 200), (5, 7, 1, 1)] {
            let b = tick_band(o, u, NOTE_H * u);
            assert!(holds(o, b), "{b:?} outside {o:?} at {u}");
        }
    }
    let o = (0, 0, 800, 100);
    assert!(holds(o, tick_band(o, 1, u32::MAX)), "a tall note is clipped");
}

#[test]
fn bounds_cover_every_outputs_band() {
    let outs = [(0, 0, 1920, 1080), (1920, 0, 2560, 1440), (4480, 0, 1280, 1024)];
    let whole = (0, 0, 5760, 1440);
    let b = tick_bounds(&outs, whole, 1, NOTE_H);
    for &o in &outs {
        assert!(holds(b, tick_band(o, 1, NOTE_H)), "{o:?}'s band outside {b:?}");
    }
    assert!(holds(whole, b));
    assert!(b.3 < whole.3, "only the rows the clocks span");
}

#[test]
fn no_output_known_is_the_whole_screens_band() {
    let whole = (0, 0, 1920, 1080);
    assert_eq!(tick_bounds(&[], whole, 1, NOTE_H), tick_band(whole, 1, NOTE_H));
}

#[test]
fn the_clock_shows_the_band_while_locked_and_tick_adds_no_second_commit() {
    let at = CLOCK.find("ctx.lock.screen.covers() { band(ctx) }").expect("locked rule");
    assert!(CLOCK[at..].contains("Changed::Part(r.0, r.1, r.2, r.3)"));
    assert!(CLOCK[at..].contains("paint_chrome_in(ctx, changed).owed().of(&[r])"));
    assert!(CLOCK[at..].contains("push_damage_commit(ctx.ports.compositor_port, rid, x, y, w, h)"));
    assert!(BAND.contains("tick_bounds(&ctx.lock.outputs, whole, px(1), note_h)"));
    assert!(!CLOCK.contains("!ctx.lock.screen.covers()"), "locked no longer means whole");
    assert!(!TICK.contains("push_damage_commit"), "refresh_clock commits the band");
}

#[test]
fn every_outputs_field_is_in_the_band_a_key_shows() {
    let outs = [(0, 0, 1920, 1080), (1920, 0, 2560, 1440), (4480, 0, 1280, 1024)];
    for u in [1, 2] {
        let b = tick_bounds(&outs, (0, 0, 5760, 1440), u, NOTE_H * u);
        for &o in &outs {
            let p = places(o, u);
            assert!(holds(b, p.field), "the field of {o:?} at {u}");
            assert!(b.1 <= p.note_y && b.1 + b.3 > p.note_y, "the note below it");
        }
    }
}

#[test]
fn a_key_in_the_field_shows_the_band_and_new_toasts_the_whole_frame() {
    assert!(CHECK.contains("crate::state::lock::Asked::Redraw => super::band::show(ctx),"));
    let show = BAND.split("pub(super) fn show(").nth(1).expect("show");
    let show = show.split("\n}\n").next().expect("its body");
    let toasts = "if ctx.toast_view.toasts_drawn != ctx.toasts.generation() {\n        return crate::server::repaint::repaint(ctx);";
    assert!(show.contains(toasts), "toasts are not in the band");
    assert!(
        show.contains("paint_chrome_in(ctx, Changed::Part(r.0, r.1, r.2, r.3)).owed().of(&[r])")
    );
    assert!(show.contains("push_damage_commit(ctx.ports.compositor_port, rid, x, y, w, h)"));
    assert!(show.find("let r = band(ctx);").is_some());
}
