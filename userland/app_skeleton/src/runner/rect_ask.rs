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

//! An app asking for its window to stand at a rect (`App::wants_rect`), as a
//! drop-down terminal does. A request: clipped to the work area (menubar to
//! dock band), and refused by the window manager without a retry until the
//! ask changes. Only a change is acted on; the rect the window had before is
//! given back when the app stops asking. An app that never asks never moves.

pub use super::work_rect::{resolve, Rect, WorkRect};

/// What the runner does with the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Stay,
    Place(Rect),
    Restore(Rect),
}

#[derive(Clone, Copy, Default)]
pub struct RectAsk {
    seen: Option<WorkRect>,
    saved: Option<Rect>,
}

impl RectAsk {
    pub const fn new() -> Self {
        Self { seen: None, saved: None }
    }

    /// The app's ask this frame, the window's rect `now`, and the work area,
    /// read only when the ask has changed.
    pub fn step(
        &mut self,
        want: Option<WorkRect>,
        now: Rect,
        work: impl FnOnce() -> Option<Rect>,
    ) -> Move {
        if want == self.seen {
            return Move::Stay;
        }
        let before = core::mem::replace(&mut self.seen, want);
        match want {
            None => self.saved.take().map_or(Move::Stay, Move::Restore),
            Some(r) => match work() {
                Some(area) => {
                    let to = resolve(r, area);
                    self.saved.get_or_insert(now);
                    if to == now {
                        Move::Stay
                    } else {
                        Move::Place(to)
                    }
                }
                None => {
                    self.seen = before; // no display answer: asked again next frame
                    Move::Stay
                }
            },
        }
    }
}
