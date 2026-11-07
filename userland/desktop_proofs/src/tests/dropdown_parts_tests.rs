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
//! Opening, closing or switching the menu bar's drop-down composes the bar
//! and the drop-down where it was and is, with its shadow, not the screen:
//! each repainted the shell whole and composed all 1920x1080 pixels twice
//! (a development guest, TCG, 352 and 679 ms).

use crate::render::layout::{menubar_rect, Rect};
use crate::taskbar::{dropdown_parts, part_frame, Area};

const SCREEN: (u32, u32) = (1920, 1080);
const FILE: Area = (195, 55, 242, 133);
const VIEW: Area = (241, 55, 220, 168);

fn area(r: Rect) -> Area {
    (r.x, r.y, r.width, r.height)
}

fn pixels(a: &[Area]) -> u64 {
    a.iter().map(|&(_, _, w, h)| u64::from(w) * u64::from(h)).sum()
}

fn body<'a>(src: &'a str, head: &str) -> &'a str {
    let at = src.find(head).expect("the function");
    let end = src[at..].find("\n}\n").expect("its end");
    &src[at..at + end]
}

#[test]
fn opening_closing_and_switching_name_the_bar_and_the_drop_downs() {
    let bar = area(menubar_rect(SCREEN.0));
    let (p, n) = dropdown_parts(bar, None, Some(FILE));
    assert_eq!(&p[..n], [bar, FILE], "opened: the bar first, the frame presented with it");
    let (p, n) = dropdown_parts(bar, Some(FILE), None);
    assert_eq!(&p[..n], [bar, FILE], "closed: where it was");
    let (p, n) = dropdown_parts(bar, Some(FILE), Some(VIEW));
    assert_eq!(&p[..n], [bar, FILE, VIEW], "switched: both places");
    let switched = pixels(&p[..n]);
    let (p, n) = dropdown_parts(bar, None, None);
    assert_eq!(&p[..n], [bar]);
    let screen = u64::from(SCREEN.0) * u64::from(SCREEN.1);
    assert!(switched * 8 < screen, "{switched} px of {screen}");
}

#[test]
fn a_switch_with_new_toasts_or_a_dock_owed_is_shown_whole() {
    let bar = area(menubar_rect(SCREEN.0));
    let (p, n) = dropdown_parts(bar, Some(FILE), Some(VIEW));
    let toasts = Some((1480, 890, 420, 96));
    assert!(part_frame(&p[..n], None, false, false).is_some(), "three fit a frame");
    assert_eq!(part_frame(&p[..n], toasts, false, false), None, "four do not: whole");
    let (p, n) = dropdown_parts(bar, None, Some(FILE));
    assert!(part_frame(&p[..n], toasts, false, false).is_some(), "an open with toasts fits");
    assert_eq!(part_frame(&p[..n], None, true, false), None, "a dock owed: whole");
    assert_eq!(part_frame(&p[..n], None, false, true), None, "locked: whole");
}

#[test]
fn the_reach_holds_the_drop_downs_shadow_as_the_right_click_menus_does() {
    const BAR: &str =
        include_str!("../../../capsule_desktop_shell/src/render/menubar_menu/frame.rs");
    const DESK: &str =
        include_str!("../../../capsule_desktop_shell/src/render/desktop_menu/frame.rs");
    let spread = "let spread = shadow_spread().saturating_add(1);";
    let shadow = "with_shadow((f.x, f.y, f.width, f.height), spread, ctx.width, ctx.height);";
    for src in [BAR, DESK] {
        let reach = body(src, "pub fn reach(");
        assert!(reach.contains("let f = frame(ctx)?;") && reach.contains(spread), "{reach}");
        assert!(reach.contains(shadow));
    }
    const PAINT: &str =
        include_str!("../../../capsule_desktop_shell/src/render/menubar_menu/paint.rs");
    let paint = body(PAINT, "pub fn paint(");
    assert!(paint.contains("shadow_panel(ctx, r, palette::R_CARD, palette::PANEL, palette::LINE);"));
    assert!(paint.contains("let r = Rect { x: ox, y: oy, width: w, height: panel_h(ctx, index) };"));
    let frame = body(BAR, "pub fn frame(");
    assert!(frame
        .contains("Some(Rect { x, y, width: panel_w(ctx, index), height: panel_h(ctx, index) })"));
    assert!(
        frame.contains("let (x, y) = origin(ctx, index);")
            && paint.contains("let (ox, oy) = origin(ctx, index);")
    );
}

#[test]
fn every_drop_down_change_but_a_row_picked_shows_its_parts() {
    const PRESS: &str = include_str!("../../../capsule_desktop_shell/src/server/input/press.rs");
    let press = body(PRESS, "pub(super) fn press(");
    let row = press.find("let row = menubar_menu::row_hit(ctx, px, py);").expect("read first");
    let click = press.find("if menubar_click::click(ctx, px, py) {").expect("the click");
    assert!(row < click, "the row and the reach are read while the drop-down is open");
    assert!(press.contains("match alone && (title || row.is_none()) {\n            true => super::super::repaint::repaint_dropdown(ctx, was),\n            false => super::super::repaint::repaint(ctx),"));
    const MOTION: &str = include_str!("../../../capsule_desktop_shell/src/server/input/motion.rs");
    assert!(MOTION.contains("None => super::super::repaint::repaint_dropdown(ctx, was),"));
    const KEY: &str =
        include_str!("../../../capsule_desktop_shell/src/server/handlers/menu_key.rs");
    let close = body(KEY, "pub(crate) fn close_menus(");
    assert!(close.contains("(Some(was), None) => repaint_dropdown(ctx, was),"));
    assert!(close.contains("(None, Some(Some(r))) => repaint_part(ctx, r),"));
    assert!(close.contains("_ => repaint(ctx),"), "both open, or nothing known: whole");
    const REPAINT: &str = include_str!("../../../capsule_desktop_shell/src/server/repaint.rs");
    let shown = body(REPAINT, "pub fn repaint_dropdown(");
    assert!(shown.contains("owed_parts(ctx, &parts[..n], dock_owed(ctx))"));
    assert!(shown.contains("show_parts(ctx, parts);"));
    const CLICK: &str =
        include_str!("../../../capsule_desktop_shell/src/server/handlers/menubar_click.rs");
    let click = body(CLICK, "pub fn click(");
    assert!(
        click.contains("if let Some(row) = row {\n        activate(ctx, title, row);"),
        "a row picked is the only activation"
    );
}
