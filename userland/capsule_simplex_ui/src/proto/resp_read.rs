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

//! Reading a `resp` object into a `Resp`.

use alloc::string::String;
use alloc::vec::Vec;

use super::chat::{chat, chat_item, ChatItem};
use super::chat_ref::ChatRef;
use super::error::error;
use super::json::Value;
use super::link::{address, link_in};
use super::resp::Resp;

fn items(v: &Value, many: &str, one: &str) -> Vec<ChatItem> {
    let one = chat_item(v.get(one));
    v.get(many).items().iter().filter_map(chat_item).chain(one).collect()
}

fn connected(v: &Value) -> Option<Resp> {
    let c = v.get("contact");
    let chat = ChatRef::direct(c.get("contactId").int()?);
    let name = c.at(&["profile", "displayName"]).str().or(c.get("localDisplayName").str());
    Some(Resp::Connected { chat, name: String::from(name.unwrap_or("")) })
}

fn request(v: &Value) -> Option<Resp> {
    let r = v.get("contactRequest");
    let name = r.at(&["profile", "displayName"]).str().or(r.get("localDisplayName").str());
    Some(Resp::Request { id: r.get("contactRequestId").int()?, name: String::from(name?) })
}

pub fn resp(v: &Value) -> Option<Resp> {
    let kind = v.get("type").str()?;
    Some(match kind {
        "apiChats" => Resp::Chats(v.get("chats").items().iter().filter_map(chat).collect()),
        "apiChat" => Resp::Chat(chat(v.get("chat"))?),
        "newChatItems" | "newChatItem" => Resp::NewItems(items(v, "chatItems", "chatItem")),
        "chatItemStatusUpdated" | "chatItemsStatusesUpdated" => {
            Resp::Status(items(v, "chatItems", "chatItem"))
        }
        "receivedContactRequest" => request(v)?,
        "contactConnected" => connected(v)?,
        "userContactLink" => Resp::Address(address(v)?),
        "userContactLinkCreated" => Resp::Address(link_in(v, "connLinkContact", "connReqContact")?),
        "invitation" => Resp::Invitation(link_in(v, "connLinkInvitation", "connReqInvitation")?),
        "sentConfirmation" | "sentInvitation" => Resp::Connecting,
        "cmdOk" | "chatCleared" | "acceptingContactRequest" => Resp::Ok,
        "chatCmdError" | "chatError" => Resp::Error(error(v)),
        other => Resp::Other(String::from(other)),
    })
}
