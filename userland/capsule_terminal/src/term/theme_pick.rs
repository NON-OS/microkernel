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

//! `theme` with no name opens a picker: Up and Down put each theme on the
//! window as it is reached, Enter keeps the one on show, and Esc goes back to
//! the one the picker opened on. The moves are here, with no window in them.

/// The themes' names, in their numbers' order.
pub const NAMES: [&[u8]; 4] = [b"dark", b"dim", b"light", b"abyss"];

/// The picker: the theme it opened on and the one on show.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pick {
    pub from: u16,
    pub at: u16,
}

/// What a key asks of the picker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Enter,
    Esc,
    Other,
}

/// What the window does: show a theme and stay open, or close showing one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Then {
    Show(u16),
    Close(u16),
    Stay,
}

impl Pick {
    pub fn open(current: u16, count: u16) -> Pick {
        let at = if current < count { current } else { 0 };
        Pick { from: at, at }
    }

    /// Move or decide; `count` themes, wrapping at both ends.
    pub fn key(&mut self, k: Key, count: u16) -> Then {
        let n = count.max(1);
        match k {
            Key::Up => {
                self.at = (self.at + n - 1) % n;
                Then::Show(self.at)
            }
            Key::Down => {
                self.at = (self.at + 1) % n;
                Then::Show(self.at)
            }
            Key::Enter => Then::Close(self.at),
            Key::Esc => Then::Close(self.from),
            Key::Other => Then::Stay,
        }
    }
}
