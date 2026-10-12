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

//! The kernel's side of a critical thermal zone (ACPI 6.5, 11.1.5): at
//! _CRT the machine must shut down. The first critical report asks the
//! desktop, as a press of the Power key, so the shutdown is orderly; if
//! the zone is still critical once the grace has passed, the kernel powers
//! off itself rather than let the hardware cook.

use core::sync::atomic::{AtomicU64, Ordering};

use super::platform_event::CRITICAL;

pub const GRACE_MS: u64 = 30_000;
const FRESH_MS: u64 = 180_000;

static CRITICAL_SINCE: AtomicU64 = AtomicU64::new(0);
static LAST: AtomicU64 = AtomicU64::new(0);
static LAST_MS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Nothing,
    AskShutdown,
    PowerOff,
}

pub fn thermal(word: u64, now_ms: u64) -> Action {
    let now_ms = now_ms.max(1);
    LAST.store(word, Ordering::Release);
    LAST_MS.store(now_ms, Ordering::Release);
    if word & CRITICAL == 0 {
        CRITICAL_SINCE.store(0, Ordering::Release);
        return Action::Nothing;
    }
    match CRITICAL_SINCE.load(Ordering::Acquire) {
        0 => {
            CRITICAL_SINCE.store(now_ms, Ordering::Release);
            Action::AskShutdown
        }
        since if now_ms.saturating_sub(since) >= GRACE_MS => Action::PowerOff,
        _ => Action::Nothing,
    }
}

/// The last thermal word, while it is under three minutes old.
pub fn latest(now_ms: u64) -> Option<u64> {
    let at = LAST_MS.load(Ordering::Acquire);
    (at != 0 && now_ms.saturating_sub(at) <= FRESH_MS).then(|| LAST.load(Ordering::Acquire))
}
