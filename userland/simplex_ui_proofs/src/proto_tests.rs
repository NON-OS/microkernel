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

//! Commands as the core reads them, and the recorded chat list.

use crate::fixture::recorded;
use crate::proto::chat_ref::{ChatKind, ChatRef};
use crate::proto::cmd::Cmd;
use crate::proto::frame::frame;
use crate::proto::item::clock;
use crate::proto::json::parse;
use crate::proto::resp::Resp;
use crate::proto::status::Status;

#[test]
fn commands_read_as_the_api_writes_them() {
    let group = ChatRef { kind: ChatKind::Group, id: 3 };
    assert_eq!(Cmd::Chats.text(), "/_get chats 1 pcc=on");
    assert_eq!(Cmd::Chat(ChatRef::direct(2)).text(), "/_get chat @2 count=50");
    assert_eq!(Cmd::Read(group).text(), "/_read chat #3");
    assert_eq!(Cmd::Connect("https://x/a#b".into()).text(), "/_connect 1 https://x/a#b");
    assert_eq!(Cmd::Invite.text(), "/_connect 1");
    assert_eq!(Cmd::ShowAddress.text(), "/_show_address 1");
    assert_eq!(Cmd::MakeAddress.text(), "/_address 1");
    assert_eq!(Cmd::Accept(4).text(), "/_accept 4");
    let send = Cmd::Send(ChatRef::direct(2), "say \"hi\"".into()).text();
    assert_eq!(send, r#"/_send @2 json [{"msgContent":{"type":"text","text":"say \"hi\""}}]"#);
    let f = parse(&frame(9, &send)).unwrap();
    assert_eq!(f.get("corrId").str(), Some("9"));
    assert_eq!(f.get("cmd").str(), Some(send.as_str()));
}

#[test]
fn the_chat_list_reads_names_last_items_and_unread_counts() {
    let Resp::Chats(chats) = recorded("/_get chats 1 pcc=on") else { panic!() };
    let names: Vec<_> = chats.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["alice", "relay ops", "bob"]);
    assert_eq!(chats[0].chat, ChatRef::direct(2));
    assert_eq!(chats[1].chat, ChatRef { kind: ChatKind::Group, id: 1 });
    assert_eq!((chats[0].unread, chats[1].unread, chats[2].unread), (2, 1, 0));
    let last = &chats[0].items[0];
    assert_eq!((last.id, last.sent, last.status), (13, false, Status::Unread));
    assert_eq!(clock(&last.ts), "09:12");
    assert_eq!(chats[1].items[0].author, "eli");
    assert_eq!(chats[2].items[0].status, Status::Sent);
}
