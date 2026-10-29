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

//! The answers and events this window reads, by their `type`.

use alloc::string::String;
use alloc::vec::Vec;

use super::chat::{ChatData, ChatItem};
use super::chat_ref::ChatRef;

#[derive(Clone, Debug, PartialEq)]
pub enum Resp {
    /// `apiChats`: every chat, each with its last item.
    Chats(Vec<ChatData>),
    /// `apiChat`: one chat with its recent items.
    Chat(ChatData),
    /// `newChatItems` (and v5's `newChatItem`): items sent or received.
    NewItems(Vec<ChatItem>),
    /// `chatItemStatusUpdated` (and `chatItemsStatusesUpdated`).
    Status(Vec<ChatItem>),
    /// `receivedContactRequest`: someone asked to connect through our address.
    Request { id: i64, name: String },
    /// `contactConnected`: a connection completed.
    Connected { chat: ChatRef, name: String },
    /// `userContactLink` or `userContactLinkCreated`: our address.
    Address(String),
    /// `invitation`: a one-time link to hand to one person.
    Invitation(String),
    /// `sentConfirmation`, `sentInvitation`: a connection we started is under way.
    Connecting,
    /// `cmdOk` and the answers that only say a command was taken.
    Ok,
    /// `chatCmdError` or `chatError`, with the error's own type names.
    Error(String),
    /// Anything else, by its type, which the window does not show.
    Other(String),
}
