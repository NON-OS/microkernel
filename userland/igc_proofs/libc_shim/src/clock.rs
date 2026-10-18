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

//! A deadline and a sleep with the capsule's API, on the host clock counted
//! as the real one is: whole milliseconds since the first look. A deadline
//! `ms` ahead can fall due up to one millisecond short, and a sleep ends
//! once the count has moved on `ms`, as the kernel's futex timeout does, so
//! both are as short here as on the machine. The tests wait the driver's
//! bounds for real, and a test's part can act on each look (`on_clock`).

use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::look::look;

static START: OnceLock<Instant> = OnceLock::new();

/// Whole milliseconds since the first look.
pub fn uptime_ms() -> u64 {
    START.get_or_init(Instant::now).elapsed().as_millis() as u64
}

pub struct Deadline {
    end_ms: u64,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        look();
        Self { end_ms: uptime_ms().saturating_add(timeout_ms) }
    }

    /// Past the deadline, and, while a device model is running
    /// (nonos_devmodel::run), only after that model has taken two more
    /// turns, as rtl8169_proofs' clock does: on a loaded builder the model
    /// thread can go unscheduled past a whole bound, and a reset it would
    /// have completed reads as a timeout. A part that never answers still
    /// times out, on the same clock. Five seconds bound the wait for a
    /// model that has stopped.
    pub fn expired(&self) -> bool {
        look();
        if uptime_ms() < self.end_ms {
            return false;
        }
        if let Some(seen) = nonos_devmodel::turns() {
            let give_up = Instant::now() + Duration::from_secs(5);
            while nonos_devmodel::turns().is_some_and(|t| t < seen + 2) && Instant::now() < give_up
            {
                std::thread::yield_now();
            }
        }
        true
    }
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    let start = *START.get_or_init(Instant::now);
    let wake = start + Duration::from_millis(uptime_ms().saturating_add(ms.max(1)));
    std::thread::sleep(wake.saturating_duration_since(Instant::now()));
    0
}
