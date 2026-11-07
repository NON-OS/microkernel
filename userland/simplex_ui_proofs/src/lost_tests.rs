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

//! A channel to the core lost once up: the window says so, keeps what it had
//! not sent, tells a new channel the network first, and starts the core again
//! a bounded number of times.

use crate::fixture::ready;
use crate::model::focus::Conn;
use crate::model::lost::LOST;
use crate::model::start::{Phase, Start, DID_NOT_START, EBUSY, EVERY_MS, LOOK_MS, MAX_STARTS};
use crate::model::start_lost::{KEEPS_CLOSING, MAX_LOSSES};
use crate::proto::chat_ref::ChatRef;
use crate::proto::cmd::Cmd;
use crate::proto::envelope::Incoming;
use crate::start_tests::launch_at;

fn send(id: i64, text: &str) -> Cmd {
    Cmd::Send(ChatRef::direct(id), text.into())
}

#[test]
fn a_flush_keeps_the_command_the_channel_went_down_on_and_all_after_it() {
    let mut m = ready();
    m.outbox = vec![send(1, "a"), send(1, "b"), send(1, "c")];
    let mut sent = Vec::new();
    let all = m.flush(|c| {
        sent.push(c.clone());
        sent.len() < 2
    });
    assert!(!all);
    assert_eq!(sent, [send(1, "a"), send(1, "b")]);
    assert_eq!(
        m.outbox,
        [send(1, "b"), send(1, "c")],
        "b may not have gone whole: it is sent again"
    );
    assert!(m.flush(|_| true));
    assert!(m.outbox.is_empty());
}

#[test]
fn a_lost_channel_is_said_and_a_new_one_is_told_the_network_before_the_person_s_messages() {
    let mut m = ready();
    assert_eq!(m.conn, Conn::Ready);
    m.outbox = vec![send(4, "still here"), Cmd::Chats, Cmd::Network, send(4, "and this")];
    m.lost();
    assert_eq!(m.conn, Conn::Offline);
    assert_eq!(m.notice, LOST);
    assert_eq!(m.outbox, [Cmd::Network, Cmd::Chats, send(4, "still here"), send(4, "and this")]);
    let chats = crate::fixture::recorded("/_get chats 1 pcc=on");
    assert!(m.apply(Incoming { corr: None, resp: chats }));
    assert_eq!(m.conn, Conn::Ready);
    assert!(m.notice.is_empty(), "the notice goes once the chats are back");
}

#[test]
fn a_lost_channel_launches_the_core_again_and_a_busy_one_is_reconnected_to() {
    let mut s = Start::new();
    let (_, at) = launch_at(&mut s, 0, 0);
    s.found();
    s.lost(at + 1000);
    assert_eq!(s.phase, Phase::Launch(None), "no name: the core has its profile");
    assert!(s.tick(at + 1000 + EVERY_MS - 1).launch.is_none(), "a whole EVERY_MS after the loss");
    let (w, again) = launch_at(&mut s, at + 1000 + EVERY_MS, EBUSY);
    assert_eq!(w, None);
    assert!(s.tick(again + EVERY_MS).look, "a running core's endpoint is looked for");
    s.found();
    assert_eq!(s.phase, Phase::Up);
}

#[test]
fn a_core_started_again_that_never_listens_is_stopped_on_not_asked_a_name() {
    let mut s = Start::new();
    let (_, at) = launch_at(&mut s, 0, 0);
    s.found();
    s.lost(at + 10);
    let mut at = at + 10 + EVERY_MS;
    for _ in 0..MAX_STARTS {
        let (w, then) = launch_at(&mut s, at, 0);
        assert_eq!(w, None, "the core has its profile");
        assert_ne!(s.phase, Phase::Ask);
        at = then + LOOK_MS;
    }
    s.tick(at);
    assert_eq!(s.phase, Phase::Stopped(DID_NOT_START));
}

#[test]
fn a_core_that_keeps_closing_is_stopped_on_unless_a_frame_comes_between() {
    let mut s = Start::new();
    let mut now = 0;
    let close = |s: &mut Start, now: &mut i64| {
        let (_, at) = launch_at(s, *now, 0);
        s.found();
        s.lost(at + 10);
        *now = at + 10 + EVERY_MS;
    };
    for _ in 0..MAX_LOSSES {
        close(&mut s, &mut now);
    }
    assert_eq!(s.phase, Phase::Launch(None), "MAX_LOSSES in a row are still started again");
    let (_, at) = launch_at(&mut s, now, 0);
    s.found();
    s.steady();
    s.lost(at + 10);
    now = at + 10 + EVERY_MS;
    assert_eq!(s.phase, Phase::Launch(None), "a frame between resets the count");
    for _ in 1..MAX_LOSSES {
        close(&mut s, &mut now);
    }
    assert_eq!(s.phase, Phase::Launch(None));
    close(&mut s, &mut now);
    assert_eq!(s.phase, Phase::Stopped(KEEPS_CLOSING));
    let step = s.tick(now + EVERY_MS);
    assert!(step.launch.is_none() && !step.look, "a stopped start asks nothing");
}

#[test]
fn a_loss_while_not_up_changes_nothing() {
    let mut s = Start::new();
    s.lost(5);
    assert_eq!(s.phase, Phase::Launch(None));
    let (_, at) = launch_at(&mut s, 0, 0);
    s.lost(at + 1);
    assert!(matches!(s.phase, Phase::Look { .. }));
}
