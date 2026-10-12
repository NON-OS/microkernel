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
//! A touchscreen's frames as pointer events. The first deliberate finger
//! down presses where it lands (INPUT_KIND_TOUCH), moving it drags
//! (POINTER_ABS) and lifting it releases (BUTTON_UP), so a screen taps,
//! drags and scrolls lists as a mouse would. Other fingers are not posted
//! yet: nothing above the input ring reads them. Positions are scaled to
//! the router's absolute range, 0 to 7FFFh.

use super::frame::Frame;

pub const ABS_MAX: u32 = 0x7FFF;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenEvent {
    Press(u32, u32),
    Move(u32, u32),
    Release,
}

#[derive(Clone, Copy, Default)]
pub struct Screen {
    /// The finger the pointer follows.
    held: Option<u32>,
}

impl Screen {
    /// The events frame `f` makes, at most two (a release and a new press
    /// when the followed finger lifts as another lands).
    pub fn step(&mut self, f: &Frame, max: (i32, i32)) -> [Option<ScreenEvent>; 2] {
        let down = || f.touches[..f.n].iter().filter(|t| t.tip && t.confidence);
        let same = self.held.and_then(|h| down().find(|t| t.id == h));
        let t = same.or_else(|| down().next());
        let at = t.map(|t| (scale(t.x, max.0), scale(t.y, max.1)));
        let events = match (self.held, t, at) {
            (Some(_), Some(_), Some((x, y))) if same.is_some() => {
                [Some(ScreenEvent::Move(x, y)), None]
            }
            (Some(_), Some(_), Some((x, y))) => {
                [Some(ScreenEvent::Release), Some(ScreenEvent::Press(x, y))]
            }
            (None, Some(_), Some((x, y))) => [Some(ScreenEvent::Press(x, y)), None],
            (Some(_), None, _) => [Some(ScreenEvent::Release), None],
            _ => [None, None],
        };
        self.held = t.map(|t| t.id);
        events
    }
}

fn scale(v: u32, max: i32) -> u32 {
    if max <= 0 {
        return 0;
    }
    (v.min(max as u32) as u64 * ABS_MAX as u64 / max as u64) as u32
}
