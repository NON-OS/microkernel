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

//! Contact requests, the compose line's commands and what Ctrl+C copies.

use crate::fixture::{event, ready, recorded};
use crate::link::script::script;
use crate::link::transcript::SESSION;
use crate::model::focus::{Focus, Key};
use crate::proto::cmd::Cmd;
use crate::proto::envelope::Incoming;
use crate::proto::resp::Resp;

#[test]
fn requests_are_accepted_from_the_compose_line() {
    let mut m = ready();
    m.outbox.clear();
    let s = script(SESSION);
    m.apply(event(s.events[3]));
    assert_eq!(m.requests, [(4, "carol".into())]);
    m.compose = "/accept".into();
    assert!(m.submit());
    assert_eq!((m.outbox.as_slice(), m.requests.len()), ([Cmd::Accept(4)].as_slice(), 0));
    m.apply(event(s.events[4]));
    assert_eq!(m.chats[0].name, "carol");
}

#[test]
fn copy_takes_a_new_link_then_the_address_then_the_last_message() {
    let mut m = ready();
    assert_eq!(m.copy_text(), Some("can you send me your new address when you have one?"));
    m.apply(Incoming { corr: None, resp: recorded("/_address 1") });
    assert!(m.copy_text().unwrap().starts_with("https://smp4.simplex.im/a#"));
    m.apply(Incoming { corr: None, resp: Resp::Invitation("simplex:/i".into()) });
    assert_eq!(m.copy_text(), Some("simplex:/i"));
}

#[test]
fn typing_in_the_list_starts_a_message_and_slash_lines_are_commands() {
    let mut m = ready();
    m.outbox.clear();
    crate::key_tests::type_text(&mut m, "/connect https://smp4.simplex.im/a#x");
    assert_eq!(m.focus, Focus::Compose);
    m.on_key(Key::Enter);
    crate::key_tests::type_text(&mut m, "/invite");
    m.on_key(Key::Enter);
    let link = Cmd::Connect("https://smp4.simplex.im/a#x".into());
    assert_eq!(m.outbox, [link, Cmd::Invite]);
}
