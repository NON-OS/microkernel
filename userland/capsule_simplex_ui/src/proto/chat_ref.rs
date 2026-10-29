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

//! Which chat a command or an item is about, as the API names it: `@<id>` for
//! a contact, `#<id>` for a group.

use alloc::format;
use alloc::string::String;

use super::json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChatKind {
    Direct,
    Group,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChatRef {
    pub kind: ChatKind,
    pub id: i64,
}

impl ChatRef {
    pub fn direct(id: i64) -> Self {
        ChatRef { kind: ChatKind::Direct, id }
    }

    /// The reference as a command writes it.
    pub fn text(&self) -> String {
        match self.kind {
            ChatKind::Direct => format!("@{}", self.id),
            ChatKind::Group => format!("#{}", self.id),
        }
    }
}

fn name_of(entity: &Value, profile: &str) -> String {
    let shown = entity.at(&[profile, "displayName"]).str();
    let local = entity.get("localDisplayName").str();
    String::from(shown.filter(|s| !s.is_empty()).or(local).unwrap_or(""))
}

/// A `chatInfo` object: the chat it names and the name to show for it. Only
/// contacts and groups are chats this window lists.
pub fn chat_info(info: &Value) -> Option<(ChatRef, String)> {
    match info.get("type").str()? {
        "direct" => {
            let c = info.get("contact");
            let id = c.get("contactId").int()?;
            Some((ChatRef::direct(id), name_of(c, "profile")))
        }
        "group" => {
            let g = info.get("groupInfo");
            let id = g.get("groupId").int()?;
            Some((ChatRef { kind: ChatKind::Group, id }, name_of(g, "groupProfile")))
        }
        _ => None,
    }
}
