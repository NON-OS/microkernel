// NONOS Operating System (AGPL-3.0-or-later)
//! What the strip proofs draw: the tab sets and strip sizes, every control
//! with the box it is drawn in, and the lock the toolkit's globals need.

use std::sync::{Mutex, MutexGuard, PoisonError};

use nonos_toolkit::widgets::{Rect, Tabs};

use crate::strip::{close, plus, tab_area, tools};
use crate::strip_spot::Spot;

static GLOBALS: Mutex<()> = Mutex::new(());

pub(crate) fn serial() -> MutexGuard<'static, ()> {
    GLOBALS.lock().unwrap_or_else(PoisonError::into_inner)
}

pub(crate) const SETS: [&[&str]; 3] = [
    &["local \u{2022} ~"],
    &["local \u{2022} ~", "shell-1 \u{2022} /usr/share/doc", "shell-2 \u{2022} ~/src"],
    &["local \u{2022} ~", "shell-1 \u{2022} /", "shell-2 \u{2022} /a", "shell-3 \u{2022} /b"],
];
pub(crate) const SIZES: [(u32, u32); 3] = [(900, 28), (420, 28), (1400, 56)];

/// Every control the strip draws, with the box it is drawn in.
pub(crate) fn drawn(names: &[&str], active: usize, (w, h): (u32, u32)) -> Vec<(Spot, Rect)> {
    let tabs = Tabs::new(names, active);
    let mut out: Vec<(Spot, Rect)> =
        tools(w, h).into_iter().enumerate().map(|(i, r)| (Spot::Tool(i), r)).collect();
    out.push((Spot::Plus, plus(&tabs, w, h, names.len())));
    for i in 0..names.len() {
        let body = tabs.tab(tab_area(w, h), i).unwrap();
        let mark = close(body).filter(|_| names.len() > 1);
        out.push((Spot::Tab(i), Rect { w: mark.map_or(body.w, |m| m.x - body.x), ..body }));
        out.extend(mark.map(|m| (Spot::Close(i), m)));
    }
    out
}
