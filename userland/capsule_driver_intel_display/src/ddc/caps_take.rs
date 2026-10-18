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

//! A capabilities read's step: ask when the bus is quiet, read once the display had time.

use super::bus::DdcBus;
use super::caps::Caps;
use super::caps_fetch::{CapsFetch, Fetch, CAPS_MAX, CAPS_REPLY_US, TRIES};
use super::caps_frame::{caps_request, fragment, CAPS_READ};
use super::error::DdcError;
use super::pace::Pacer;

impl CapsFetch {
    /// One step at `now`; whether a message went or came.
    fn step(&mut self, b: &mut dyn DdcBus, now: u64, free: bool) -> bool {
        match self.state {
            Fetch::Got(_) | Fetch::Refused => false,
            Fetch::Ready if !free => false,
            Fetch::Ready => {
                let sent = b.send(&caps_request(self.len as u16));
                self.state = if sent.is_ok() { Fetch::Asked(now) } else { Fetch::Refused };
                true
            }
            Fetch::Asked(at) if now.saturating_sub(at) < CAPS_REPLY_US => false,
            Fetch::Asked(_) => {
                let mut r = [0u8; CAPS_READ];
                let got = b.receive(&mut r).and_then(|_| fragment(self.len as u16, &r));
                take(self, got);
                true
            }
        }
    }
}

/// The reply asked for: more string, its end, or a miss (asked again until TRIES run out).
fn take(f: &mut CapsFetch, got: Result<&[u8], DdcError>) {
    f.state = match got {
        Ok([]) => Caps::parse(&f.text[..f.len]).map_or(Fetch::Refused, Fetch::Got),
        Ok(d) if f.len + d.len() > CAPS_MAX => Fetch::Refused,
        Ok(d) => {
            f.text[f.len..f.len + d.len()].copy_from_slice(d);
            (f.len, f.misses) = (f.len + d.len(), 0);
            Fetch::Ready
        }
        Err(_) if f.misses + 1 >= TRIES => Fetch::Refused,
        Err(_) => {
            f.misses += 1;
            Fetch::Ready
        }
    };
}

impl Pacer {
    /// One step of this output's capabilities read at `now`, never waiting;
    /// a message sent or read starts the bus's quiet window.
    pub fn fetch_caps(&mut self, b: &mut dyn DdcBus, now: u64) {
        let free = self.next_at(now) <= now;
        if self.caps.step(b, now, free) {
            self.sent(now);
        }
    }
}
