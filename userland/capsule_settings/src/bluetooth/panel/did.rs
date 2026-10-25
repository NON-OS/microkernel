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

//! What a request the page made did, taken into the panel.

use nonos_bt_proto::ops::{E_OK, OP_ANSWER, OP_PAIR};

use super::known::Panel;
use super::notice::{done, refused};
use super::request::Request;

impl Panel {
    /// The service answered `req` with `status` at `now_ms`. An answer sent
    /// wipes its digits, taken or not: the next question wants fresh ones.
    pub fn did(&mut self, req: &Request, status: i32, now_ms: i64) {
        if req.op == OP_ANSWER {
            self.digits.clear();
        }
        if status != E_OK {
            self.notice = Some(refused(status));
            return;
        }
        self.notice = Some(done(req));
        if req.op == OP_PAIR {
            self.window_from_ms = Some(now_ms);
        }
    }
}
