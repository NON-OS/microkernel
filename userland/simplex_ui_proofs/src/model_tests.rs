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

//! What the core's answers and events do to the model.

use crate::fixture::{event, ready};
use crate::link::script::script;
use crate::link::transcript::SESSION;
use crate::model::focus::Conn;
use crate::proto::chat_ref::{ChatKind, ChatRef};
use crate::proto::cmd::Cmd;
use crate::proto::status::Status;

const ALICE: ChatRef = ChatRef { kind: ChatKind::Direct, id: 2 };
const OPS: ChatRef = ChatRef { kind: ChatKind::Group, id: 1 };

#[test]
fn the_list_opens_the_newest_chat_and_reads_it() {
    let m = ready();
    assert_eq!(m.conn, Conn::Ready);
    assert_eq!(m.selected, Some(ALICE));
    assert_eq!(m.outbox, [Cmd::Chat(ALICE), Cmd::Read(ALICE)]);
    assert_eq!(m.current().unwrap().unread, 0);
    assert_eq!(m.chats[1].unread, 1);
}

#[test]
fn new_items_count_unread_and_move_their_chat_up() {
    let mut m = ready();
    m.outbox.clear();
    let s = script(SESSION);
    assert!(m.apply(event(s.events[1])));
    assert_eq!((m.chats[0].chat, m.chats[0].unread), (OPS, 2));
    assert_eq!(
        m.chats[0].last().unwrap().text,
        "cert rotated on relay 2, clients reconnected fine"
    );
    assert!(m.outbox.is_empty());
    m.apply(event(s.events[0]));
    assert_eq!((m.chats[0].chat, m.chats[0].unread), (ALICE, 0));
    assert_eq!(m.outbox, [Cmd::Read(ALICE)]);
    m.apply(event(s.events[2]));
    let bob = &m.chats[m.find(ChatRef::direct(3)).unwrap()];
    assert_eq!(bob.last().unwrap().status, Status::Delivered);
}
