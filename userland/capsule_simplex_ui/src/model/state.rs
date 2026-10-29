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

//! The whole of what the window shows, and the commands it has yet to send.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::chat::Chat;
use super::focus::{Conn, Focus};
use crate::proto::chat_ref::ChatRef;
use crate::proto::cmd::Cmd;

#[derive(Debug)]
pub struct Model {
    /// Most recent first.
    pub chats: Vec<Chat>,
    pub selected: Option<ChatRef>,
    pub focus: Focus,
    pub compose: String,
    pub search: String,
    /// The name asked for when the core has no profile yet (start.rs).
    pub name: String,
    pub conn: Conn,
    /// Our address, once the core said it.
    pub address: Option<String>,
    /// Contact requests not yet accepted, by id and name.
    pub requests: Vec<(i64, String)>,
    /// One line for the last thing the core said that is not a chat item.
    pub notice: String,
    /// Commands for the transport, oldest first.
    pub outbox: Vec<Cmd>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            chats: Vec::new(),
            selected: None,
            focus: Focus::List,
            compose: String::new(),
            search: String::new(),
            name: String::new(),
            conn: Conn::Offline,
            address: None,
            requests: Vec::new(),
            notice: String::new(),
            outbox: vec![Cmd::Chats, Cmd::ShowAddress],
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::new()
    }
}
