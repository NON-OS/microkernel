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

//! The transcript as a `Transport`: answers from the recording, made-up
//! answers to sends, and the recorded events pushed one per `EVENT_EVERY`
//! ticks once the chat list was asked for.

use alloc::string::String;

use super::echo::echo;
use super::transcript::{frame, Transcript, EVENT_EVERY, NOT_RECORDED};
use super::Transport;
use crate::proto::json::parse;

impl Transport for Transcript {
    fn send(&mut self, sent: &str) {
        let Some(v) = parse(sent) else { return };
        let (corr, Some(cmd)) = (v.get("corrId").str(), v.get("cmd").str()) else { return };
        self.started |= cmd.starts_with("/_get chats");
        if let Some(answers) = self.script.answers(cmd) {
            let framed = answers.iter().map(|a| frame(corr, a));
            self.inbox.extend(framed);
        } else if let Some((answer, after)) = echo(cmd, self.next_id) {
            self.next_id += 1;
            self.inbox.push_back(frame(corr, &answer));
            self.later.push_front(after);
        } else if cmd.starts_with("/_read ") {
            self.inbox.push_back(frame(corr, "{\"type\":\"cmdOk\"}"));
        } else {
            self.inbox.push_back(frame(corr, NOT_RECORDED));
        }
    }

    fn recv(&mut self) -> Option<String> {
        self.inbox.pop_front()
    }

    fn tick(&mut self) {
        self.ticks += self.started as u32;
        if self.ticks >= EVENT_EVERY {
            self.ticks = 0;
            if let Some(e) = self.later.pop_front() {
                self.inbox.push_back(frame(None, &e));
            }
        }
    }

    fn up(&self) -> bool {
        true
    }

    fn label(&self) -> &'static str {
        "transcript"
    }
}
