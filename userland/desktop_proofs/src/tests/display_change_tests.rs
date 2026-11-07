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

//! The display changes size while the desktop runs: the compositor moves it
//! from the boot framebuffer to the display driver (compositor
//! setup/adopt.rs). The window manager placed every window against the size
//! it read at its own start, which on a fast machine (KVM) is the boot
//! framebuffer's, while each app clamps its own moves to the compositor's
//! size now. The rect the window manager hit-tests then stopped being the
//! rect drawn: a press on a window's title bar reached the app in the wrong
//! place, as an edge or the content, and a click on its middle could miss
//! it. The window manager now reads the size again before it places a window
//! (state/display_watch.rs, server/display_refresh.rs).

use nonos_toolkit::decorations::{edges_at, hit_test_at, DecorationHit};

use crate::focus::topmost_hit_at;
use crate::geometry::{clamp_to_display, Rect};
use crate::window::{Kind, Visibility, Window, WindowTable};
use crate::wm_display_watch::{DisplayWatch, RECHECK_MS};

const BOOT_FB: (u32, u32) = (1280, 800);
const DRIVER: (u32, u32) = (1920, 1080);
/// The scale an app draws its frame at on the driver's display.
const QUARTERS: u32 = 5;
const APP: u32 = 30;

/// The window manager's placing of a window move: the size it reads when its
/// watch says the read is due, else the size it holds, and the rect clamped
/// to that, as server/handlers/window_move.rs clamps it.
struct Wm {
    size: (u32, u32),
    watch: DisplayWatch,
    refreshes: bool,
}

impl Wm {
    fn started(at_ms: i64, compositor: (u32, u32), refreshes: bool) -> Wm {
        let mut watch = DisplayWatch::new();
        watch.read(at_ms);
        Wm { size: compositor, watch, refreshes }
    }

    fn place(&mut self, now_ms: i64, compositor: (u32, u32), r: Rect) -> Rect {
        if self.refreshes && self.watch.due(now_ms) {
            self.watch.read(now_ms);
            self.size = compositor;
        }
        clamp_to_display(r, self.size.0, self.size.1)
    }
}

fn table(rect: Rect) -> WindowTable {
    let mut t = WindowTable::new();
    let w = Window {
        owner_pid: APP,
        window_id: 1,
        rect,
        kind: Kind::Normal,
        visibility: Visibility::Visible,
        z: 1,
        in_use: true,
        full_screen: false,
        parent: 0,
        grab: false,
    };
    t.insert(w).expect("room");
    t
}

/// The app's own move, clamped to the compositor's display (app_skeleton
/// window_ops/move_window.rs), and the window manager's record of it.
fn moved(wm: &mut Wm) -> (Rect, Rect) {
    let drawn = Rect { x: 960, y: 560, width: 842, height: 421 };
    let max = (DRIVER.0 - drawn.width, DRIVER.1 - drawn.height);
    let drawn = Rect { x: drawn.x.min(max.0), y: drawn.y.min(max.1), ..drawn };
    (drawn, wm.place(2_000, DRIVER, drawn))
}

/// What the app makes of a press at screen (sx, sy), in the window
/// coordinates the input router gives it from the window manager's hit test.
fn press(t: &WindowTable, sx: u32, sy: u32) -> Option<(DecorationHit, bool)> {
    let hit = topmost_hit_at(t, sx, sy, 0)?;
    let (lx, ly) = (hit.local_x, hit.local_y);
    let (w, h) = (842, 421);
    Some((
        hit_test_at(w, h, false, lx, ly, QUARTERS),
        edges_at(w, h, false, lx, ly, QUARTERS).any(),
    ))
}

#[test]
fn a_size_held_from_the_boot_framebuffer_puts_the_window_elsewhere_than_drawn() {
    let mut wm = Wm::started(0, BOOT_FB, false);
    let (drawn, recorded) = moved(&mut wm);
    assert_ne!(recorded, drawn, "the old size clamped the record of the move");
    let t = table(recorded);
    // A press 32 down the drawn title bar is not the title bar to the app.
    let on_title = press(&t, drawn.x + 120, drawn.y + 32);
    assert_ne!(on_title, Some((DecorationHit::Titlebar, false)), "{on_title:?}");
    // And a click on the drawn window's lower right misses every window.
    let corner = (drawn.x + drawn.width - 40, drawn.y + drawn.height - 40);
    assert!(topmost_hit_at(&t, corner.0, corner.1, 0).is_none());
}

#[test]
fn the_window_manager_places_on_the_size_the_compositor_has_now() {
    let mut wm = Wm::started(0, BOOT_FB, true);
    let (drawn, recorded) = moved(&mut wm);
    assert_eq!(recorded, drawn, "the record is the rect drawn");
    let t = table(recorded);
    assert_eq!(press(&t, drawn.x + 120, drawn.y + 32), Some((DecorationHit::Titlebar, false)));
    let middle = (drawn.x + drawn.width / 2, drawn.y + drawn.height / 2);
    let hit = topmost_hit_at(&t, middle.0, middle.1, 0).expect("on the window");
    assert_eq!((hit.local_x, hit.local_y), (drawn.width / 2, drawn.height / 2));
}

#[test]
fn a_drag_reads_the_size_at_most_once_a_recheck() {
    let mut watch = DisplayWatch::new();
    assert!(watch.due(0), "never read: read now");
    watch.read(0);
    let reads = (0..1_000).filter(|ms| {
        watch.due(*ms) && {
            watch.read(*ms);
            true
        }
    });
    // A move every millisecond for a second asks about ten times.
    assert_eq!(reads.count() as i64, 1_000 / RECHECK_MS - 1);
    // A clock that ran back is not a reason to trust the size.
    assert!(watch.due(-5));
}
