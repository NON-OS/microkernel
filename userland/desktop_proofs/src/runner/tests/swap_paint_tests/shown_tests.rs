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

use super::new_path::New;
use super::rig::{content, App, Old, H, W};
use nonos_toolkit::decorations::Rect;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

/// A frame's damage, in content coordinates: some empty, some past the
/// content's edge, as an app's `take_damage` may say.
fn damage(s: &mut u64) -> Rect {
    let c = content();
    let r = xorshift(s);
    let (x, y) = ((r % (c.w as u64 + 8)) as u32, ((r >> 16) % (c.h as u64 + 8)) as u32);
    let (w, h) = (((r >> 32) % 40) as u32, ((r >> 48) % 30) as u32);
    Rect { x, y, w, h }
}

fn fresh(app: &App) -> Vec<u32> {
    let mut px = vec![0; (W * H) as usize];
    app.draw_whole(&mut px);
    px
}

/// One frame of the model app on both paths: a frame colour change paints
/// whole, else a content change paints its damage, as an app that paints
/// its damage alone does or as one that paints all its content (the
/// Terminal's frames for its banner's uptime row).
fn frame(s: &mut u64, app: &mut App, old: &mut Old, new: &mut New) {
    if xorshift(s).is_multiple_of(10) {
        app.frame = 0xFF00_0000 | (xorshift(s) as u32 & 0x00FF_FFFF);
        old.whole(app);
        new.whole(app);
        return;
    }
    let r = damage(s);
    let cut = super::rig::clip(r);
    app.change(cut, 0xFF00_0000 | (xorshift(s) as u32 & 0x00FF_FFFF));
    old.partial(app, r);
    if !new.primed {
        new.whole(app);
    } else if !new.partial(app, r, !xorshift(s).is_multiple_of(3)) {
        new.primed = false;
    }
}

#[test]
fn the_screen_matches_the_single_surface_frame_for_frame() {
    for seed in 1..=40u64 {
        let mut s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let (mut app, mut old, mut new) = (App::new(), Old::new(), New::blank());
        old.whole(&app);
        assert!(new.whole(&app), "the first frame, in the buffer not submitted");
        assert_ne!(new.shown, 0);
        new.tick();
        for step in 0..120 {
            frame(&mut s, &mut app, &mut old, &mut new);
            new.tick();
            assert!(new.screen == old.screen, "seed {seed} step {step}: the screens differ");
            assert!(new.bufs[new.shown] == fresh(&app), "seed {seed} step {step}: shown not whole");
        }
    }
}

#[test]
fn a_compositor_slow_to_release_is_never_drawn_over_and_catches_up_whole() {
    for seed in 1..=40u64 {
        let mut s = seed.wrapping_mul(0xD1B5_4A32_D192_ED03) | 1;
        let (mut app, mut old, mut new) = (App::new(), Old::new(), New::blank());
        new.whole(&app);
        for _ in 0..120 {
            new.hold = (xorshift(&mut s) % 4) as u32;
            new.refuse_next = xorshift(&mut s).is_multiple_of(13);
            let shown_before = new.bufs[new.shown].clone();
            let shown = new.shown;
            frame(&mut s, &mut app, &mut old, &mut new);
            if new.shown == shown {
                assert!(
                    new.bufs[shown] == shown_before,
                    "seed {seed}: the shown buffer was written"
                );
            }
            new.tick();
        }
        // Released and painted whole again, it shows the app as it is.
        (new.hold, new.refuse_next) = (0, false);
        for _ in 0..4 {
            new.tick();
        }
        assert!(new.whole(&app), "seed {seed}: a buffer is free again");
        assert!(new.screen == fresh(&app), "seed {seed}: the screen caught up");
    }
}

/// The runner judges a buffer against the rect the app paints, and tells
/// the app to draw everything before it draws the whole window.
#[test]
fn a_buffer_out_of_date_beyond_the_damage_is_drawn_whole_and_the_app_told() {
    const PARTIAL: &str = include_str!("../../../../../app_skeleton/src/runner/paint_partial.rs");
    assert!(PARTIAL.contains("if whole_needed(stale, (d.x, d.y, d.width, d.height)) {"));
    assert!(PARTIAL.contains("        booted.app.paint_whole_next();\n        draw_whole("));
    assert!(PARTIAL.contains("let drawn = match booted.app.painted_damage_alone() {"));
    assert!(PARTIAL.contains("finish_band(&mut fb, maximized, corner_box(drawn));"));
    const BROWSER: &str = include_str!("../../../../../capsule_browser/src/browser/app/damage.rs");
    assert!(BROWSER.contains(
        "pub(super) fn whole_next(state: &mut State) {\n    state.track.painting = Damage::FULL;"
    ));
    const TERMINAL: &str =
        include_str!("../../../../../capsule_terminal/src/term/terminal/app_impl.rs");
    assert!(TERMINAL.contains("fn paint_whole_next(&mut self) {"));
    assert!(TERMINAL.contains("fn painted_damage_alone(&self) -> bool {\n        self.row_painted"));
    const ROW: &str =
        include_str!("../../../../../capsule_terminal/src/term/terminal/app_impl_paint.rs");
    assert!(ROW.contains("self.row_painted = self.paint_line_only(fb);"));
    const BROWSER_PAINT: &str =
        include_str!("../../../../../capsule_browser/src/browser/app/paint.rs");
    assert!(BROWSER_PAINT.contains("state.track.painted_all = parts.has(Damage::FULL);"));
}
