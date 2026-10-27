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

//! One chat item, from a `chatItem` object: who wrote it, its text, when, and
//! how far it got.

use alloc::string::String;

use super::json::Value;
use super::status::Status;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub id: i64,
    /// Written here, not received.
    pub sent: bool,
    /// The member who wrote it, in a group; empty in a direct chat.
    pub author: String,
    pub text: String,
    /// The item's time as the core writes it, ISO 8601 in UTC.
    pub ts: String,
    pub status: Status,
}

fn text_of(item: &Value) -> String {
    let content = item.at(&["content", "msgContent", "text"]).str();
    let meta = item.at(&["meta", "itemText"]).str();
    String::from(content.or(meta).unwrap_or(""))
}

pub fn item(item: &Value) -> Option<Item> {
    let meta = item.get("meta");
    let dir = item.get("chatDir");
    let sent = matches!(dir.get("type").str()?, "directSnd" | "groupSnd" | "localSnd");
    let author = dir.at(&["groupMember", "localDisplayName"]).str().unwrap_or("");
    Some(Item {
        id: meta.get("itemId").int()?,
        sent,
        author: String::from(author),
        text: text_of(item),
        ts: String::from(meta.get("itemTs").str().unwrap_or("")),
        status: Status::from_json(meta.get("itemStatus")),
    })
}

/// The `HH:MM` of an item's time, or empty when it is not one.
pub fn clock(ts: &str) -> &str {
    ts.get(11..16).filter(|hm| hm.as_bytes().get(2) == Some(&b':')).unwrap_or("")
}
