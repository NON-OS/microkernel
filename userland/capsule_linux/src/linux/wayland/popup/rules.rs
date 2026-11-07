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

//! The rules a popup's grab is held to. Pure, so the host proofs hold it.
//!
//! A grab is taken only with the serial of the latest press this client
//! was sent, or of a key's release sent after it, by a popup whose parent
//! is the topmost grabbing popup, or the shown toplevel when none is open.
//! A grab's end takes down every popup above it first, topmost first. A
//! popup with no grab (a tooltip) is shown too, from its first commit
//! (show.rs), and the pointer reaches it, but the keyboard does not.

use alloc::vec::Vec;

use super::stack::{Popup, Popups};

impl Popups {
    /// Whether popup `id` may take a grab with `serial`, its window's
    /// toplevel surface being `root` (None when no window is shown). One
    /// already shown may not: a grab comes before the popup is mapped, and
    /// granted then it would be opened a second time.
    pub fn may_grab(&self, id: u32, serial: u32, root: Option<u32>) -> bool {
        let fresh = |p: &&Popup| p.id == id && !p.grabbed && !p.shown;
        let Some(p) = self.list.iter().find(fresh) else { return false };
        let Some(root) = root else { return false };
        let input = self.last_press == Some(serial) || self.last_key_up == Some(serial);
        input && p.parent == self.top().map_or(root, |t| t.surface)
    }

    /// A press, key or button, sent with `serial`: the one a grab names
    /// from now on, a release before it no longer.
    pub fn pressed(&mut self, serial: u32) {
        self.last_press = Some(serial);
        self.last_key_up = None;
    }

    /// No input since the grabs ended may open one.
    pub fn forget_input(&mut self) {
        self.last_press = None;
        self.last_key_up = None;
    }

    /// What a grab's end on `id` takes down: every grabbing popup from it
    /// up, topmost first; or, when it holds no grab, `id` and every popup
    /// placed on it however deep, children first, so none is left on a
    /// parent that is gone.
    pub fn ending(&self, id: u32) -> Vec<u32> {
        let grabbed: Vec<u32> = self.list.iter().filter(|p| p.grabbed).map(|p| p.id).collect();
        if let Some(from) = grabbed.iter().position(|g| *g == id) {
            return grabbed[from..].iter().rev().copied().collect();
        }
        let mut out: Vec<u32> = self.list.iter().filter(|p| p.id == id).map(|p| p.id).collect();
        let mut at = 0;
        while at < out.len() {
            let Some(surface) = self.list.iter().find(|p| p.id == out[at]).map(|p| p.surface) else {
                break;
            };
            let children = self.list.iter().filter(|p| p.parent == surface && !out.contains(&p.id));
            let more: Vec<u32> = children.map(|p| p.id).collect();
            out.extend(more);
            at += 1;
        }
        out.reverse();
        out
    }

    /// Every popup, the grabbing ones topmost first, then the waiting ones.
    pub fn all(&self) -> Vec<u32> {
        let waiting = self.list.iter().filter(|p| !p.grabbed).map(|p| p.id);
        self.list.iter().rev().filter(|p| p.grabbed).map(|p| p.id).chain(waiting).collect()
    }

    /// The shown popup under (`x`, `y`) on the toplevel's surface, topmost
    /// first, and the point on its own surface.
    pub fn under(&self, x: i32, y: i32) -> Option<(u32, (i32, i32))> {
        let inside = |p: &&Popup| {
            let (px, py, w, h) = p.rect;
            let (dx, dy) = (i64::from(x) - i64::from(px), i64::from(y) - i64::from(py));
            dx >= 0 && dy >= 0 && dx < i64::from(w) && dy < i64::from(h)
        };
        let p = self.list.iter().rev().filter(|p| p.shown).find(inside)?;
        Some((p.surface, (x.wrapping_sub(p.rect.0), y.wrapping_sub(p.rect.1))))
    }
}
