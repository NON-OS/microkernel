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

//! What an answer or an event from the core does to the model.

use alloc::format;
use alloc::string::String;

use super::chat::Chat;
use super::focus::Conn;
use super::says::{CONNECTING, LINK_NOTICE, NO_ADDRESS};
use super::state::Model;
use crate::proto::envelope::Incoming;
use crate::proto::resp::Resp;

impl Model {
    /// Applies one frame; true when the window must repaint.
    pub fn apply(&mut self, msg: Incoming) -> bool {
        match msg.resp {
            Resp::Chats(list) => {
                self.conn = Conn::Ready;
                for data in list {
                    self.take_chat(data, false);
                }
                if self.selected.is_none() {
                    self.selected = self.chats.first().map(|c| c.chat);
                    self.open_selected();
                }
            }
            Resp::Chat(data) => {
                let opened = Some(data.chat) == self.selected;
                self.take_chat(data, true);
                if opened {
                    self.mark_read();
                }
            }
            Resp::NewItems(items) => self.new_items(items),
            Resp::Status(items) => self.statuses(items),
            Resp::Request { id, name } => {
                self.notice = format!("{name} asks to connect; type /accept to take it");
                self.requests.push((id, name));
            }
            Resp::Connected { chat, name } => {
                self.notice = format!("{name} is connected");
                if self.find(chat).is_none() {
                    self.chats.insert(0, Chat::new(chat, name));
                }
            }
            Resp::Address(link) => self.address = Some(link),
            Resp::Invitation(link) => self.notice = format!("{LINK_NOTICE}{link}"),
            Resp::Connecting => self.notice = String::from(CONNECTING),
            Resp::Error(e) if e.ends_with("userContactLinkNotFound") => {
                self.notice = String::from(NO_ADDRESS);
            }
            Resp::Error(e) => self.notice = format!("the core said: {e}"),
            Resp::Ok | Resp::Other(_) => return false,
        }
        true
    }
}
