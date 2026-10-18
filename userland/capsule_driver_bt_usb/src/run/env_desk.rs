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

//! The user's part, through the desk: what the link tells NBTC, and what
//! NBTC hands back from Settings.

use super::env::CapsuleEnv;
use super::say::say;
use crate::link::{Answer, Asked, Desk};
use crate::nbtc::Part;
use nonos_libc::mk_uptime_ms;

impl Desk for CapsuleEnv {
    fn ready(&mut self, keyboard: bool, mouse: bool, keys_post: bool) {
        self.desk.board().ready(keyboard, mouse, keys_post);
    }
    fn linked(&mut self, identity_type: u8, identity: [u8; 6], fresh: bool) {
        self.desk.board().linked((identity_type, identity), fresh);
    }
    fn asked(&mut self, asked: Asked) {
        let part = match asked {
            Asked::Passkey => Part::Enter,
            Asked::Number(n) => Part::Confirm(n),
        };
        say(&[b"pairing waits on the user, in Settings"]);
        self.desk.board().ask(part);
    }
    fn answer(&mut self) -> Option<Answer> {
        self.desk.board().take_answer()
    }
    fn may_pair(&mut self) -> bool {
        self.desk.board().may_pair(mk_uptime_ms().max(0) as u64)
    }
    fn hang_up(&mut self) -> bool {
        core::mem::take(&mut self.desk.board().drop_link)
    }
    fn radio_off(&mut self) -> bool {
        !self.desk.board().scan
    }
}
