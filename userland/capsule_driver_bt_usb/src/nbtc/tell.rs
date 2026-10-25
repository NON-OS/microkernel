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

//! What the link tells the board, through the capsule's `Env`.

use super::board::Board;
use super::ops::{IDLE, SCANNING};
use super::part::{Identity, Level, Part, LEVELS_MAX};
use crate::link::Answer;
use nonos_bt_hogp::host::Battery;

impl<K> Board<K> {
    /// HID is ready on the link: pairing asks nothing more of the user.
    pub fn ready(&mut self, keyboard: bool, mouse: bool, keys_post: bool) {
        (self.ready, self.part, self.answer) =
            (Some((keyboard, mouse, keys_post)), Part::None, None);
    }

    /// The link runs on this bond. A bond pairing just made uses up the
    /// pairing window: one start lets one device in.
    pub fn linked(&mut self, who: Identity, fresh: bool) {
        self.current = Some(who);
        if fresh {
            self.pair_until = 0;
        }
    }

    /// The link is over: nothing of it is left to report.
    pub fn ended(&mut self) {
        (self.ready, self.current, self.part, self.answer) = (None, None, Part::None, None);
        self.drop_link = false;
        self.state = if self.scan { SCANNING } else { IDLE };
        self.levels.retain(|l| l.who.is_some());
    }

    /// A level for the bond the link runs on. A level for a Battery Service
    /// already kept replaces it.
    pub fn battery(&mut self, b: Battery) {
        let who = self.current;
        let same = |l: &Level| l.who == who && l.instance == b.instance;
        match self.levels.iter().position(same) {
            Some(i) => self.levels[i].percent = b.percent,
            None if self.levels.len() < LEVELS_MAX => {
                self.levels.push(Level { who, instance: b.instance, percent: b.percent })
            }
            None => {}
        }
    }

    pub fn ask(&mut self, part: Part) {
        (self.part, self.answer) = (part, None);
    }

    /// The user's answer, once: the link acts on it only once.
    pub fn take_answer(&mut self) -> Option<Answer> {
        self.answer.take()
    }

    pub fn may_pair(&self, now_ms: u64) -> bool {
        now_ms < self.pair_until
    }
}
