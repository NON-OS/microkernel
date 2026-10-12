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
//! _CRT the machine must shut down, through the critical watch; and the
//! last thermal word, for whoever asks how hot the machine is.

use core::sync::atomic::{AtomicU64, Ordering};

use super::critical_watch::{Action, Watch};
use super::platform_event::CRITICAL;

pub use super::critical_watch::GRACE_MS;

const FRESH_MS: u64 = 180_000;

static WATCH: Watch = Watch::new();
static LAST: AtomicU64 = AtomicU64::new(0);
static LAST_MS: AtomicU64 = AtomicU64::new(0);

pub fn thermal(word: u64, now_ms: u64) -> Action {
    LAST.store(word, Ordering::Release);
    LAST_MS.store(now_ms.max(1), Ordering::Release);
    WATCH.step(word & CRITICAL != 0, now_ms)
}

/// The last thermal word, while it is under three minutes old.
pub fn latest(now_ms: u64) -> Option<u64> {
    let at = LAST_MS.load(Ordering::Acquire);
    (at != 0 && now_ms.saturating_sub(at) <= FRESH_MS).then(|| LAST.load(Ordering::Acquire))
}
