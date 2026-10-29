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

//! The window from the keyboard alone.

use crate::fixture::ready;
use crate::model::focus::Focus;
use crate::model::focus::Key;
use crate::proto::chat_ref::{ChatKind, ChatRef};
use crate::proto::cmd::Cmd;

const OPS: ChatRef = ChatRef { kind: ChatKind::Group, id: 1 };

pub fn type_text(m: &mut crate::model::state::Model, text: &str) {
    for ch in text.chars() {
        assert!(m.on_key(Key::Char(ch)));
    }
}

#[test]
fn up_and_down_move_through_the_chats_and_open_them() {
    let mut m = ready();
    m.outbox.clear();
    assert!(!m.on_key(Key::Up));
    assert!(m.on_key(Key::Down));
    assert_eq!(m.selected, Some(OPS));
    assert_eq!(m.outbox, [Cmd::Chat(OPS), Cmd::Read(OPS)]);
    assert!(m.on_key(Key::Down));
    assert!(!m.on_key(Key::Down));
    assert_eq!(m.current().unwrap().name, "bob");
}

#[test]
fn tab_composes_and_enter_sends_to_the_open_chat() {
    let mut m = ready();
    m.outbox.clear();
    assert!(m.on_key(Key::Tab));
    assert_eq!(m.focus, Focus::Compose);
    type_text(&mut m, "hello, ä");
    assert!(m.on_key(Key::Backspace));
    assert!(m.on_key(Key::Enter));
    assert_eq!(m.outbox, [Cmd::Send(ChatRef::direct(2), "hello,".into())]);
    assert!(m.compose.is_empty());
    assert!(!m.on_key(Key::Enter));
    assert!(m.on_key(Key::Esc));
    assert_eq!(m.focus, Focus::List);
    assert!(!m.on_key(Key::Esc));
}

#[test]
fn ctrl_k_searches_and_enter_takes_the_match() {
    let mut m = ready();
    assert!(m.on_key(Key::Search));
    type_text(&mut m, "BO");
    assert_eq!(m.visible().len(), 1);
    assert_eq!(m.current().unwrap().name, "bob");
    assert!(m.on_key(Key::Enter));
    assert_eq!((m.focus, m.search.as_str(), m.visible().len()), (Focus::Compose, "", 3));
}
