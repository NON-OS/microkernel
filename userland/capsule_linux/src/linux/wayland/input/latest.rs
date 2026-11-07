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

//! The pointer motion a backed-up client was not sent. A client that has
//! left a mebibyte unread is sent no motion (pump.rs), but the latest one is
//! kept: it goes before the next button, key or wheel, so a click lands
//! where the pointer is and not where the client last heard it was, and it
//! goes once the client has read its queue down. A newer motion replaces
//! it. Pure, so a host proof holds it (capsule_linux_proofs,
//! motion_kept_tests).

pub struct Latest<T> {
    kept: Option<T>,
}

impl<T> Latest<T> {
    pub fn new() -> Latest<T> {
        Latest { kept: None }
    }

    /// What to send for `event`, in order: the kept motion first, when one
    /// must go before it, then the event itself, unless it is kept instead.
    pub fn offer(&mut self, event: T, motion: bool, backlogged: bool) -> [Option<T>; 2] {
        match (motion, backlogged) {
            (true, true) => {
                self.kept = Some(event);
                [None, None]
            }
            (true, false) => {
                self.kept = None;
                [None, Some(event)]
            }
            (false, _) => [self.kept.take(), Some(event)],
        }
    }

    /// The kept motion, once the client is no longer backed up.
    pub fn release(&mut self, backlogged: bool) -> Option<T> {
        if backlogged {
            return None;
        }
        self.kept.take()
    }
}
