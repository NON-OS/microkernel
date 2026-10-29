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

//! The commands this window sends, in the chat API's own syntax.

use alloc::format;
use alloc::string::String;

use super::chat_ref::ChatRef;
use super::json::push_quoted;

/// The core's first user; the window shows one profile.
pub const USER_ID: u32 = 1;
/// How many recent items opening a chat asks for.
pub const PAGE: u32 = 50;

#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    /// `/_get chats 1 pcc=on`: every chat with its last item.
    Chats,
    /// `/_get chat @<id> count=50`.
    Chat(ChatRef),
    /// `/_send @<id> json [...]`: one text message.
    Send(ChatRef, String),
    /// `/_read chat @<id>`: mark the chat read.
    Read(ChatRef),
    /// `/_connect 1 <link>`: connect through someone's link.
    Connect(String),
    /// `/_connect 1`: make a one-time invitation link.
    Invite,
    /// `/_show_address 1`: our address, if one was made.
    ShowAddress,
    /// `/_address 1`: make our address.
    MakeAddress,
    /// `/_accept <id>`: accept a contact request.
    Accept(i64),
}

impl Cmd {
    pub fn text(&self) -> String {
        match self {
            Cmd::Chats => format!("/_get chats {USER_ID} pcc=on"),
            Cmd::Chat(c) => format!("/_get chat {} count={PAGE}", c.text()),
            Cmd::Send(c, text) => {
                let mut body = String::new();
                push_quoted(&mut body, text);
                format!(
                    "/_send {} json [{{\"msgContent\":{{\"type\":\"text\",\"text\":{body}}}}}]",
                    c.text()
                )
            }
            Cmd::Read(c) => format!("/_read chat {}", c.text()),
            Cmd::Connect(link) => format!("/_connect {USER_ID} {link}"),
            Cmd::Invite => format!("/_connect {USER_ID}"),
            Cmd::ShowAddress => format!("/_show_address {USER_ID}"),
            Cmd::MakeAddress => format!("/_address {USER_ID}"),
            Cmd::Accept(id) => format!("/_accept {id}"),
        }
    }
}
