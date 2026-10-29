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

//! The compose line: text for the open chat, or one of the few commands a
//! person types, since the window has no menus.

use alloc::string::String;

use super::command::command;
use super::focus::Focus;
use super::state::Model;
use crate::proto::cmd::Cmd;

/// The longest message the line takes, in bytes. The core's own limit is
/// larger; this keeps one line on screen reasonable.
pub const MAX_COMPOSE: usize = 4096;

impl Model {
    /// Sends the compose line, or runs it when it is a command; true when it
    /// went somewhere.
    pub fn submit(&mut self) -> bool {
        let line = String::from(self.compose.trim());
        if line.is_empty() {
            return false;
        }
        if line == "/accept" {
            let Some((id, _)) = self.requests.first().cloned() else { return false };
            self.requests.remove(0);
            self.outbox.push(Cmd::Accept(id));
        } else if let Some(cmd) = command(&line) {
            self.outbox.push(cmd);
        } else {
            let Some(chat) = self.selected else { return false };
            self.outbox.push(Cmd::Send(chat, line));
        }
        self.compose.clear();
        true
    }

    /// Adds text to the field that has the keys, within its limit.
    pub fn insert(&mut self, text: &str) {
        let (field, max) = match self.focus {
            Focus::Search => (&mut self.search, MAX_COMPOSE),
            Focus::Name => (&mut self.name, super::start_name::NAME_MAX),
            _ => (&mut self.compose, MAX_COMPOSE),
        };
        for ch in text.chars().filter(|c| !c.is_control()) {
            if field.len() + ch.len_utf8() > max {
                break;
            }
            field.push(ch);
        }
    }
}
