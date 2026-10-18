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

//! The bounded bring-up schedule: how many attempts, and the sleep
//! between them. Pure, so it is proven on the host.

/// Attempts for a loop that counts its tries; bring_up runs by the clock.
pub const BRINGUP_ATTEMPTS: u32 = 7;
/// Sleep after the first failed attempt.
pub const BRINGUP_FIRST_DELAY_MS: u64 = 100;
/// No single sleep is longer than this.
pub const BRINGUP_MAX_DELAY_MS: u64 = 3_200;

/// What to do after attempt number `attempts_done` (counting from 1) failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Next {
    /// Sleep this many milliseconds, then try again.
    Retry(u64),
    /// Stop trying.
    GiveUp,
}

/// The sleep after the `attempts_done`th failure, doubling from the first
/// delay and held at the cap. Zero attempts is not a failure and waits for
/// nothing.
pub const fn delay_after(attempts_done: u32) -> u64 {
    if attempts_done == 0 {
        return 0;
    }
    let shift = attempts_done - 1;
    if shift >= 32 {
        return BRINGUP_MAX_DELAY_MS;
    }
    let delay = BRINGUP_FIRST_DELAY_MS.saturating_mul(1u64 << shift);
    if delay > BRINGUP_MAX_DELAY_MS {
        BRINGUP_MAX_DELAY_MS
    } else {
        delay
    }
}

/// The decision after a failed attempt.
pub const fn next(attempts_done: u32) -> Next {
    if attempts_done >= BRINGUP_ATTEMPTS {
        Next::GiveUp
    } else {
        Next::Retry(delay_after(attempts_done))
    }
}

/// The longest a driver spends asleep before it gives up: the sum of every
/// sleep the schedule allows.
pub const fn total_sleep_ms() -> u64 {
    let mut total = 0u64;
    let mut done = 1u32;
    while done < BRINGUP_ATTEMPTS {
        total += delay_after(done);
        done += 1;
    }
    total
}
