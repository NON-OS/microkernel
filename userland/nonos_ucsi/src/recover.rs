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

//! A PPM that stuck or could not be reached, walked again from a reset:
//! at once, then after `BACKOFF_MS`, twice that, and so on, until
//! `MAX_RESETS` walks in a row have failed and the session stops asking.

use super::offered::Offered;
use super::ppm::Ppm;
use super::session::{Session, State, BACKOFF_MS, MAX_RESETS};
use super::start::start;
use super::status::Status;

impl Session {
    pub(super) fn recover(&mut self, ppm: &mut impl Ppm, now: u64, failures: u8) {
        let mut pending = 0;
        match start(ppm, &mut pending) {
            Ok(ports) => {
                let before = core::mem::replace(&mut self.ports, ports);
                for (i, p) in self.ports.iter().enumerate() {
                    if before.get(i).map(|b| b.status.map(|s| s.state()))
                        != Some(p.status.map(|s| s.state()))
                    {
                        self.changed |= 1u128 << (i + 1);
                    }
                }
                (self.pending, self.state) = (pending, State::Running);
            }
            Err(_) if failures + 1 >= MAX_RESETS => self.state = State::Failed,
            Err(_) => {
                let wait = BACKOFF_MS << failures;
                let at = now.saturating_add(wait);
                self.state = State::Recovering { failures: failures + 1, at };
            }
        }
    }

    pub(super) fn update(&mut self, n: u8, s: Option<Status>, offered: Offered) {
        let Some(port) = self.ports.get_mut(usize::from(n) - 1) else { return };
        if port.status.map(|s| s.state()) != s.map(|s| s.state()) || port.offered != offered {
            self.changed |= 1u128 << n;
        }
        (port.status, port.offered) = (s, offered);
    }

    /// The connectors whose status changed since the last call.
    pub fn take_changed(&mut self) -> u128 {
        core::mem::take(&mut self.changed)
    }
}
