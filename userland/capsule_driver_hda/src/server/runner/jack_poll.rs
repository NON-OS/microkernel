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
//! Switching between speakers and headphones while the driver serves.
//!
//! The headphone jack is read twice a second, and at once when the codec
//! has sent an unsolicited response since the last read (a jack event, if
//! firmware left a pin's reporting enabled). A change programs every
//! output's pin for the new state (`codec::jack::apply`): speakers off with
//! headphones in, back on when they come out. A jack read that fails is
//! tried again later, later each time it fails, and a codec that answers
//! none of a bounded run of them is logged once and not read again
//! (`jack_schedule`): the outputs stay as they were last set.

use super::jack_schedule::{JackSchedule, JackState};
use crate::clock::now_ms;
use crate::controller::codec::jack;
use crate::setup::{Driver, Line};

pub(super) struct JackPoll(JackSchedule);

impl JackPoll {
    pub(super) fn new() -> Self {
        JackPoll(JackSchedule::new(now_ms()))
    }

    pub(super) fn poll(&mut self, d: &mut Driver) {
        if !self.0.due(now_ms(), d.link.unsolicited()) {
            return;
        }
        self.0.answered(follow(d));
        if self.0.state() == JackState::Lost {
            Line::new("[HDA] codec stopped answering; headphone jack no longer read").emit();
        }
    }
}

/// Read the jack and switch the outputs if it changed. False when any verb
/// went unanswered, which is what the schedule backs off from.
fn follow(d: &mut Driver) -> bool {
    let Ok(plugged) = jack::plugged(&mut d.link, &d.codec, &d.plan) else { return false };
    if plugged == d.status.plugged {
        return true;
    }
    if jack::apply(&mut d.link, &d.codec, &d.plan, plugged).is_err() {
        return false;
    }
    d.status.plugged = plugged;
    let what = if plugged { "in, speakers off" } else { "out, speakers on" };
    Line::new("[HDA] headphones ").s(what).emit();
    true
}
