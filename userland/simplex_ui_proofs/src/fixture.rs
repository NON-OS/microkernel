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

//! The recorded session's answers, read as the window reads a frame.

use crate::link::script::script;
use crate::link::transcript::Transcript;
use crate::link::transcript::SESSION;
use crate::link::Transport;
use crate::model::state::Model;
use crate::proto::cmd::Cmd;
use crate::proto::envelope::{incoming, Incoming};
use crate::proto::frame::frame;
use crate::proto::resp::Resp;

/// The first recorded answer to `cmd`.
pub fn recorded(cmd: &str) -> Resp {
    let s = script(SESSION);
    let answer = s.answers(cmd).unwrap()[0];
    incoming(&format!("{{\"corrId\":\"7\",\"resp\":{answer}}}")).unwrap().resp
}

/// A model that has read the recorded chat list.
pub fn ready() -> Model {
    let mut m = Model::new();
    assert_eq!(m.outbox, [Cmd::Chats, Cmd::ShowAddress]);
    m.outbox.clear();
    assert!(m.apply(Incoming { corr: None, resp: recorded("/_get chats 1 pcc=on") }));
    m
}

pub fn event(resp: &str) -> Incoming {
    incoming(&format!("{{\"resp\":{resp}}}")).unwrap()
}

/// The capsule's pump: commands out as frames, answers and events in.
pub fn pump(m: &mut Model, t: &mut Transcript, corr: &mut u32) {
    loop {
        for cmd in core::mem::take(&mut m.outbox) {
            *corr += 1;
            t.send(&frame(*corr, &cmd.text()));
        }
        let Some(text) = t.recv() else { break };
        m.apply(incoming(&text).unwrap());
    }
}

/// `n` window ticks, each followed by a pump.
pub fn ticks(m: &mut Model, t: &mut Transcript, corr: &mut u32, n: u32) {
    for _ in 0..n {
        t.tick();
        pump(m, t, corr);
    }
}
