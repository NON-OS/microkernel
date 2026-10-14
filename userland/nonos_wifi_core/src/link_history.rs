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

//! Why the link last ended, and how the last join came out, kept by a driver
//! and sent at the end of the control family's link reply, after the AKM:
//! `[ended u8][joins u8][code i32 LE]`. Ended is 0, not ended; 1, lost (the
//! access point left, went unheard, or let its SA Query go unanswered); 2,
//! left on request. Joins counts the joins finished this boot (0 before the
//! first, then 1 to 255, wrapping past 0), so a reader can tell one ended
//! since it last looked; code is that join's result. net.core's autojoin
//! rejoins only a lost link, never one the person left.

/// The tail's length.
pub const TAIL_LEN: usize = 6;

/// Why the link last ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ended {
    No = 0,
    Lost = 1,
    Left = 2,
}

/// What a driver remembers of its link and its last join.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LinkHistory {
    pub ended: Ended,
    pub joins: u8,
    pub code: i32,
}

impl LinkHistory {
    pub const NEW: Self = Self { ended: Ended::No, joins: 0, code: 0 };

    /// A join finished with `code`; a joined one clears how the last ended.
    pub fn joined(&mut self, code: i32) {
        self.joins = self.joins.checked_add(1).unwrap_or(1);
        self.code = code;
        if code == 0 {
            self.ended = Ended::No;
        }
    }

    /// The link was lost, not left.
    pub fn lost(&mut self) {
        self.ended = Ended::Lost;
    }

    /// The person left the network.
    pub fn left(&mut self) {
        self.ended = Ended::Left;
    }

    /// Write the tail into `out`, returning its length.
    pub fn encode(&self, out: &mut [u8]) -> Option<usize> {
        let tail = out.get_mut(..TAIL_LEN)?;
        tail[0] = self.ended as u8;
        tail[1] = self.joins;
        tail[2..].copy_from_slice(&self.code.to_le_bytes());
        Some(TAIL_LEN)
    }
}
