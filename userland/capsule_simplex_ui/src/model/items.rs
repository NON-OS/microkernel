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

//! New items and status changes.

use alloc::vec::Vec;

use super::chat::Chat;
use super::state::Model;
use crate::proto::chat::ChatItem;
use crate::proto::status::Status;

impl Model {
    pub(super) fn new_items(&mut self, items: Vec<ChatItem>) {
        let mut read_open = false;
        for ci in items {
            let at = self.find(ci.chat).unwrap_or_else(|| {
                self.chats.push(Chat::new(ci.chat, ci.name.clone()));
                self.chats.len() - 1
            });
            let mut chat = self.chats.remove(at);
            let incoming = !ci.item.sent && ci.item.status == Status::Unread;
            if chat.put(ci.item) && incoming {
                if Some(chat.chat) == self.selected {
                    read_open = true;
                } else {
                    chat.unread += 1;
                }
            }
            self.chats.insert(0, chat);
        }
        if read_open {
            self.mark_read();
        }
    }

    pub(super) fn statuses(&mut self, items: Vec<ChatItem>) {
        for ci in items {
            if let Some(at) = self.find(ci.chat) {
                self.chats[at].set_status(ci.item.id, ci.item.status);
            }
        }
    }
}
