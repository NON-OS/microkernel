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

//! A chat with its items, from the objects `apiChats` lists and `apiChat`
//! answers with, and the `AChatItem` pairs events carry.

use alloc::string::String;
use alloc::vec::Vec;

use super::chat_ref::{chat_info, ChatRef};
use super::item::{item, Item};
use super::json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct ChatData {
    pub chat: ChatRef,
    pub name: String,
    /// Oldest first. `apiChats` with `pcc=on` sends the last item alone.
    pub items: Vec<Item>,
    pub unread: u32,
}

pub fn chat(v: &Value) -> Option<ChatData> {
    let (chat, name) = chat_info(v.get("chatInfo"))?;
    let unread = v.at(&["chatStats", "unreadCount"]).int().unwrap_or(0);
    Some(ChatData {
        chat,
        name,
        items: v.get("chatItems").items().iter().filter_map(item).collect(),
        unread: unread.clamp(0, u32::MAX as i64) as u32,
    })
}

/// An item with the chat it belongs to.
#[derive(Clone, Debug, PartialEq)]
pub struct ChatItem {
    pub chat: ChatRef,
    pub name: String,
    pub item: Item,
}

pub fn chat_item(v: &Value) -> Option<ChatItem> {
    let (chat, name) = chat_info(v.get("chatInfo"))?;
    Some(ChatItem { chat, name, item: item(v.get("chatItem"))? })
}
