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

//! A condition the machine must not stay in: a thermal zone at _CRT, a
//! battery at its critical level while discharging. The first report asks
//! the desktop to shut down, as a press of the Power key, so the shutdown
//! is orderly; if the condition still holds once the grace has passed,
//! the kernel powers off itself. A report without it resets the watch.

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
}

impl Default for Watch {
    fn default() -> Self {
        Self::new()
    }
}
