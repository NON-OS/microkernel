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

//! A condition the machine must not stay in: a thermal zone at _HOT or
//! _CRT, a battery critical while discharging. The first report asks the
//! desktop to shut down, as a Power key press, so the shutdown is orderly;
//! if the condition still holds once the grace has passed, the kernel
//! powers off itself. The grace runs on the clock (F-108): `tick` acts on
//! a watch its last report armed though no capsule reports again. A
//! restart is not a reading; only a reading without the condition clears.

use core::sync::atomic::{AtomicU64, Ordering};

pub const GRACE_MS: u64 = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Nothing,
    AskShutdown,
    PowerOff,
}

pub struct Watch {
    since: AtomicU64,
}

impl Watch {
    pub const fn new() -> Self {
        Watch { since: AtomicU64::new(0) }
    }

    pub fn step(&self, critical: bool, now_ms: u64) -> Action {
        let now_ms = now_ms.max(1);
        if !critical {
            self.since.store(0, Ordering::Release);
            return Action::Nothing;
        }
        match self.since.load(Ordering::Acquire) {
            0 => {
                self.since.store(now_ms, Ordering::Release);
                Action::AskShutdown
            }
            since if now_ms.saturating_sub(since) >= GRACE_MS => Action::PowerOff,
            _ => Action::Nothing,
        }
    }

    /// The clock alone: a watch its last report armed, past its grace, is
    /// acted on as if that reading still held; any other is left be.
    pub fn tick(&self, now_ms: u64) -> Action {
        match self.since.load(Ordering::Acquire) {
            since if since != 0 && now_ms.saturating_sub(since) >= GRACE_MS => Action::PowerOff,
            _ => Action::Nothing,
        }
    }
}

impl Default for Watch {
    fn default() -> Self {
        Self::new()
    }
}
