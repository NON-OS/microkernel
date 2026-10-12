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

//! Configuration: every Protocol Mode is set to boot with a Write Without
//! Response, as HIDS 1.0 asks of that characteristic, and each report's
//! notifications are turned on by writing its CCCD (Vol 3, Part G,
//! 4.12.3), one Write Request at a time.

use super::phase::Phase;
use super::state::Host;
use super::step::{Outcome, Step};
use super::types::Kind;
use crate::uuids::{BOOT_PROTOCOL, NOTIFY_ON};
use alloc::vec::Vec;
use nonos_bt_wire::att;

impl Host {
    pub(super) fn enable(&mut self, source: usize, now_ms: u64) -> Step {
        // The mode writes go once, ahead of the first CCCD write.
        let modes = || self.modes.iter().map(|&h| att::write(h, &[BOOT_PROTOCOL], true)).collect();
        let mut send: Vec<Vec<u8>> = if source == 0 { modes() } else { Vec::new() };
        let Some(cccd) = self.sources.get(source).and_then(|s| s.cccd) else {
            return self.ready(send);
        };
        self.phase = Phase::Enable { source };
        let mut step = self.request(att::write(cccd, &NOTIFY_ON.to_le_bytes(), false), now_ms);
        if step.outcome.is_none() {
            send.append(&mut step.send);
            step.send = send;
        }
        step
    }

    pub(super) fn enabled(&mut self, source: usize, now_ms: u64) -> Step {
        self.enable(source + 1, now_ms)
    }

    fn ready(&mut self, mut send: Vec<Vec<u8>>) -> Step {
        self.phase = Phase::Ready;
        // A lock set before now: the peripheral's LEDs start off.
        if self.locks.bits() != 0 {
            send.append(&mut self.leds());
        }
        let has = |k| self.sources.iter().any(|s| s.kind == k);
        let outcome = Outcome::Ready { keyboard: has(Kind::Keyboard), mouse: has(Kind::Mouse) };
        Step { send, outcome: Some(outcome), ..Step::default() }
    }
}
