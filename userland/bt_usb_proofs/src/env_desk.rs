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

//! The user's part in the proofs' environment: what the link told it, and
//! the answers a proof scripts for it.

use std::collections::VecDeque;

use crate::env::TestEnv;
use crate::link::{Answer, Asked, Desk};

pub struct User {
    pub may_pair: bool,
    /// Answers given in turn, each when the link polls for one.
    pub answers: VecDeque<Answer>,
    /// With no answer scripted: wait (true) or decline at once (false, as
    /// the capsule did before NBTC could ask the user).
    pub keep_waiting: bool,
    pub asked: Vec<Asked>,
    pub ready: Vec<(bool, bool, bool)>,
    pub linked: Vec<(u8, [u8; 6], bool)>,
    pub hang_up: bool,
    /// The radio turned off (E-72): a live link ends and nothing more is sent.
    pub off: bool,
}

impl User {
    pub fn new() -> Self {
        let (answers, asked, ready, linked) = (VecDeque::new(), vec![], vec![], vec![]);
        let hang_up = false;
        Self { may_pair: true, answers, keep_waiting: false, asked, ready, linked, hang_up, off: false }
    }
}

impl Desk for TestEnv {
    fn ready(&mut self, keyboard: bool, mouse: bool, keys_post: bool) {
        self.user.ready.push((keyboard, mouse, keys_post));
    }
    fn linked(&mut self, identity_type: u8, identity: [u8; 6], fresh: bool) {
        self.user.linked.push((identity_type, identity, fresh));
    }
    fn asked(&mut self, asked: Asked) {
        self.user.asked.push(asked);
    }
    fn answer(&mut self) -> Option<Answer> {
        match self.user.answers.pop_front() {
            Some(a) => Some(a),
            None if self.user.keep_waiting => None,
            None => Some(Answer::Cancel),
        }
    }
    fn may_pair(&mut self) -> bool {
        self.user.may_pair
    }
    fn hang_up(&mut self) -> bool {
        std::mem::take(&mut self.user.hang_up)
    }
    fn radio_off(&mut self) -> bool {
        self.user.off
    }
}
