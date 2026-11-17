// NONOS Operating System (AGPL-3.0-or-later)
//! What the rail proofs draw: the lists, the column they sit in, every
//! control with the content box it is drawn in, and a frame of the lists.

use nonos_app_skeleton::{InputEvent, InputKind, PaintBuffer};
use nonos_toolkit::widgets::{Pointer, Rect, Scrolled};

use crate::rail_column::Column;
use crate::rail_lists::{draw_lists, Entry};
use crate::rail_nav::nav;
use crate::rail_scroll::{column, RailFit};
use crate::rail_spot::{row, Spot};

pub(crate) const RAIL: Rect = Rect { x: 0, y: 40, w: 320, h: 420 };

pub(crate) const SESSIONS: [Entry; 3] = [
    Entry { name: "local", path: "/home/user", busy: false },
    Entry { name: "shell-1", path: "/usr/share/doc/nonos", busy: true },
    Entry { name: "shell-2", path: "/", busy: false },
];
pub(crate) const PROJECTS: [Entry; 2] = [
    Entry { name: "src", path: "/home/user/src", busy: false },
    Entry { name: "notes", path: "/home/user/notes", busy: false },
];

/// The rail scrolled by `scroll`, clamped as the Terminal clamps it.
pub(crate) fn col(scroll: u32) -> Column {
    column(RAIL, scroll, RailFit { sessions: 3, projects: 2, telemetry: true })
}

/// Every control the lists draw, with its box in content pixels.
pub(crate) fn drawn(c: &Column) -> Vec<(Spot, Rect)> {
    let n = nav(c.rect.w, c.sessions, c.projects);
    let mut out = vec![(Spot::NewSession, n.s_plus), (Spot::AddProject, n.p_plus)];
    out.extend((0..c.sessions).map(|i| (Spot::Session(i), row(n.s_list, i))));
    out.extend((0..c.projects).map(|i| (Spot::Project(i), row(n.p_list, i))));
    out
}

/// The window point at the middle of content box `b`.
pub(crate) fn mid(c: &Column, b: Rect) -> (i32, i32) {
    let y = (c.rect.y + b.y + b.h / 2) as i32 - c.off as i32;
    ((c.rect.x + b.x + b.w / 2) as i32, y)
}

pub(crate) fn ev(kind: InputKind, (x, y): (i32, i32)) -> InputEvent {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns }
}

/// The lists as the rail paints them, `h` rows of the column tall.
pub(crate) fn frame(c: &Column, ptr: &Pointer, active: usize, h: u32) -> Vec<u32> {
    let w = c.rect.w;
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    draw_lists(&mut Scrolled::new(&mut fb, c.off), c, ptr, &SESSIONS, &PROJECTS, active);
    px
}

/// The points, in the rail's own pixels, where two frames differ.
pub(crate) fn changed(a: &[u32], b: &[u32], w: u32) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n as u32 % w) as i32, (n as u32 / w) as i32);
    a.iter().zip(b).enumerate().filter(|(_, (p, q))| p != q).map(|(n, _)| at(n)).collect()
}
