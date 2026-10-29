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

//! Opening a chat and reading it.

use super::state::Model;
use crate::proto::cmd::Cmd;
use crate::proto::status::Status;

impl Model {
    /// Marks the open chat read, here and in the core, if anything is unread.
    pub fn mark_read(&mut self) {
        let Some(at) = self.selected.and_then(|c| self.find(c)) else { return };
        let chat = &mut self.chats[at];
        let mut any = chat.unread > 0;
        chat.unread = 0;
        for item in chat.items.iter_mut().filter(|i| i.status == Status::Unread) {
            item.status = Status::Read;
            any = true;
        }
        if any {
            self.outbox.push(Cmd::Read(chat.chat));
        }
    }

    /// Asks for the open chat's recent items the first time it opens.
    pub fn open_selected(&mut self) {
        let Some((chat, loaded)) = self.current().map(|c| (c.chat, c.loaded)) else { return };
        if !loaded {
            self.outbox.push(Cmd::Chat(chat));
        }
        self.mark_read();
    }
}
