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

//! A link's life after the connection is up. Each turn takes one event
//! and one ACL packet if the controller has them and sends what waits, so
//! no turn blocks; an idle turn pauses the transport about a millisecond.
//! The loop ends when the link does: the peer leaves, a timer runs out,
//! or this host gives up. A failing USB transfer, or a controller that
//! lost its state, ends it with that error. However it ends, every key and
//! button still down goes up.

use super::answered::OFF;
use super::end::End;
use super::env::Env;
use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};

pub fn serve<T: Transport, E: Env>(
    host: &mut Host<T>,
    s: &mut Session,
    env: &mut E,
) -> Result<End, HciError> {
    let end = turns(host, s, env);
    s.release(env);
    end
}

fn turns<T: Transport, E: Env>(
    host: &mut Host<T>,
    s: &mut Session,
    env: &mut E,
) -> Result<End, HciError> {
    loop {
        if let Some(end) = s.end {
            return Ok(end);
        }
        // Turned off: end the link before this turn sends anything (E-72).
        if env.radio_off() {
            s.give_up(OFF);
            return Ok(End::GaveUp(OFF));
        }
        let now = env.now_ms();
        env.tick(now);
        s.timers(host, now, env)?;
        s.alive(host, now)?;
        s.flush(host)?;
        let event = host.next_event()?;
        if let Some(e) = &event {
            s.on_event(host, e)?;
        }
        let acl = host.transport().poll_acl()?;
        if let Some(a) = &acl {
            s.on_acl(host, a, env)?;
        }
        if event.is_none() && acl.is_none() {
            host.transport().pause();
        }
    }
}
