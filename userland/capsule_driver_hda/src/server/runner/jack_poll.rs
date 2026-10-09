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
//! skipped and tried at the next poll.

use crate::clock::now_ms;
use crate::controller::codec::jack;
use crate::setup::{Driver, Line};

const POLL_MS: u64 = 500;

pub(super) struct JackPoll {
    last: u64,
    events: u32,
}

impl JackPoll {
    pub(super) fn new() -> Self {
        JackPoll { last: now_ms(), events: 0 }
    }

    pub(super) fn poll(&mut self, d: &mut Driver) {
        let now = now_ms();
        let events = d.link.unsolicited();
        if now.saturating_sub(self.last) < POLL_MS && events == self.events {
            return;
        }
        self.last = now;
        self.events = events;
        let Ok(plugged) = jack::plugged(&mut d.link, &d.codec, &d.plan) else { return };
        if plugged == d.status.plugged {
            return;
        }
        if jack::apply(&mut d.link, &d.codec, &d.plan, plugged).is_ok() {
            d.status.plugged = plugged;
            let what = if plugged { "in, speakers off" } else { "out, speakers on" };
            Line::new("[HDA] headphones ").s(what).emit();
        }
    }
}
