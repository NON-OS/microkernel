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

//! The user's part: the passkey typed, or the yes or no on the numbers
//! both sides show. A call in the wrong state is refused, as a caller bug.

use super::initiator::Initiator;
use super::io::Random;
use super::state::State;
use super::step::Step;
use crate::error::SmpError;
use crate::pdu::{NUMERIC_COMPARISON_FAILED, PASSKEY_ENTRY_FAILED};

impl Initiator {
    /// The six digits the user typed, or `None` if they gave up.
    pub fn passkey(
        &mut self,
        value: Option<u32>,
        rng: &mut dyn Random,
        now_ms: u64,
    ) -> Result<Step, SmpError> {
        if self.state != State::AwaitPasskey {
            return Err(SmpError::Refused);
        }
        if self.expired(now_ms) {
            return Ok(self.time_out());
        }
        self.last_ms = now_ms;
        Ok(match value.filter(|&v| v < 1_000_000) {
            Some(v) => {
                self.passkey = v;
                self.round(0, rng)
            }
            None => self.fail(PASSKEY_ENTRY_FAILED),
        })
    }

    /// Whether the user saw the same six digits on both devices.
    pub fn confirm(&mut self, same: bool, now_ms: u64) -> Result<Step, SmpError> {
        if self.state != State::AwaitUserConfirm {
            return Err(SmpError::Refused);
        }
        if self.expired(now_ms) {
            return Ok(self.time_out());
        }
        self.last_ms = now_ms;
        Ok(if same { self.check() } else { self.fail(NUMERIC_COMPARISON_FAILED) })
    }
}
