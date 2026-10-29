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

//! The window's loop over the recorded session, as the capsule's pump runs
//! it: commands out as frames, answers and events in.

use crate::fixture::{pump, ticks};
use crate::link::transcript::{Transcript, EVENT_EVERY, SESSION};
use crate::link::Transport;
use crate::model::state::Model;
use crate::proto::envelope::incoming;
use crate::proto::frame::frame;
use crate::proto::status::Status;

#[test]
fn the_session_opens_the_first_chat_with_its_items() {
    let (mut m, mut t, mut corr) = (Model::new(), Transcript::new(SESSION), 0);
    pump(&mut m, &mut t, &mut corr);
    let chat = m.current().unwrap();
    assert!(chat.loaded);
    assert_eq!(chat.items.len(), 4);
    assert_eq!(m.notice, "no address yet; type /address to make one");
    ticks(&mut m, &mut t, &mut corr, EVENT_EVERY);
    assert_eq!(m.current().unwrap().items.len(), 5);
}

#[test]
fn a_send_echoes_then_is_delivered() {
    let (mut m, mut t, mut corr) = (Model::new(), Transcript::new(SESSION), 0);
    pump(&mut m, &mut t, &mut corr);
    m.compose = "on my way".into();
    assert!(m.submit());
    pump(&mut m, &mut t, &mut corr);
    let last = m.current().unwrap().last().unwrap().clone();
    assert_eq!((last.text.as_str(), last.sent, last.status), ("on my way", true, Status::Sent));
    ticks(&mut m, &mut t, &mut corr, EVENT_EVERY);
    assert_eq!(m.current().unwrap().last().unwrap().status, Status::Delivered);
}

#[test]
fn what_was_not_recorded_is_an_error() {
    let mut t = Transcript::new(SESSION);
    t.send(&frame(3, "/_get chat @99 count=50"));
    let answer = incoming(&t.recv().unwrap()).unwrap();
    assert_eq!(answer.corr.as_deref(), Some("3"));
    assert!(matches!(answer.resp, crate::proto::resp::Resp::Error(_)));
    assert!(t.recv().is_none());
}
