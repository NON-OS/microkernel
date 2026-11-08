// NONOS Operating System (AGPL-3.0-or-later)
//! What the palette proofs draw: entries shaped like the Terminal's index,
//! the bodies and queries they are drawn with, and where each row is hit.

use nonos_app_skeleton::{InputEvent, InputKind, PaintBuffer};
use nonos_toolkit::widgets::Rect;

use crate::palette::{Action, Entry, Kind, Palette};
use crate::palette_paint::draw;
use crate::palette_view::View;
use crate::verbs::VERBS;

pub(crate) const W: u32 = 1000;
pub(crate) const H: u32 = 600;
pub(crate) const BODIES: [Rect; 2] =
    [Rect { x: 0, y: 0, w: 900, h: 560 }, Rect { x: 180, y: 40, w: 640, h: 420 }];
pub(crate) const NEEDLES: [&str; 5] = ["", "e", "the", "SESS", "zz"];

/// The verbs, then a session and the actions, in the order the index adds them.
pub(crate) fn entries() -> Vec<Entry<'static>> {
    let entry = |kind, label, hint, action| Entry { kind, label, hint, action };
    let run = |&(label, hint): &(&'static str, &'static str)| {
        entry(Kind::Command, label, hint, Action::Run)
    };
    let mut v: Vec<Entry> = VERBS.iter().map(run).collect();
    v.push(entry(Kind::History, "ls -la", "run again", Action::Run));
    v.push(entry(Kind::Session, "src", "switch to", Action::Session(1)));
    v.push(entry(Kind::Project, "nonos", "enter", Action::Project(0)));
    v.push(entry(Kind::Action, "New Session", "another tab", Action::NewSession));
    v.push(entry(Kind::Action, "Toggle Monitor", "the telemetry rail", Action::ToggleMonitor));
    v.push(entry(Kind::Action, "Change Theme", "the next palette", Action::ChangeTheme));
    v
}

pub(crate) fn typed(needle: &str) -> Palette {
    let mut pal = Palette::new();
    pal.show();
    needle.bytes().for_each(|b| pal.push(b));
    pal
}

pub(crate) fn paint(body: Rect, pal: &Palette, e: &[Entry]) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    draw(&mut fb, body, pal, e);
    px
}

/// The middle of the band a click on row `i` lands in.
pub(crate) fn mid(v: &View, i: usize) -> (i32, i32) {
    let x = (v.at.x + v.at.w / 2) as i32;
    let on = |y: &i32| v.menu.hit(v.at, x, *y) == Some(i);
    let ys: Vec<i32> = (v.at.y as i32..(v.at.y + v.at.h) as i32).filter(on).collect();
    (x, (ys[0] + ys[ys.len() - 1]) / 2)
}

/// Every pixel that differs between two frames, with the row a click there hits.
pub(crate) fn changed(v: &View, a: &[u32], b: &[u32]) -> Vec<Option<usize>> {
    let at = |n: usize| ((n as u32 % W) as i32, (n as u32 / W) as i32);
    let diff = a.iter().zip(b).enumerate().filter(|(_, (p, q))| p != q);
    diff.map(|(n, _)| at(n)).map(|(x, y)| v.menu.hit(v.at, x, y)).collect()
}

/// Each scale, body and query the drawing proofs run over.
pub(crate) fn cases() -> impl Iterator<Item = (u32, Rect, &'static str)> {
    [4, 8]
        .into_iter()
        .flat_map(|q| BODIES.into_iter().flat_map(move |b| NEEDLES.map(|n| (q, b, n))))
}

pub(crate) fn ev(kind: InputKind, (x, y): (i32, i32)) -> InputEvent {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns }
}
