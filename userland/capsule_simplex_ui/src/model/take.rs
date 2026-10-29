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

//! Finding a chat, and taking one the core listed or answered with.

use super::chat::Chat;
use super::state::Model;
use crate::proto::chat::ChatData;
use crate::proto::chat_ref::ChatRef;

impl Model {
    pub fn find(&self, chat: ChatRef) -> Option<usize> {
        self.chats.iter().position(|c| c.chat == chat)
    }

    pub fn current(&self) -> Option<&Chat> {
        self.chats.get(self.find(self.selected?)?)
    }

    pub(super) fn take_chat(&mut self, data: ChatData, loaded: bool) {
        let at = self.find(data.chat).unwrap_or_else(|| {
            self.chats.push(Chat::new(data.chat, data.name.clone()));
            self.chats.len() - 1
        });
        let chat = &mut self.chats[at];
        chat.name = data.name;
        chat.unread = data.unread;
        chat.loaded |= loaded;
        for item in data.items {
            chat.put(item);
        }
    }
}
