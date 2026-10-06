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

//! How long a driver keeps trying a device that is present but will not come
//! up, and what it does when it stops.
//!
//! Every driver used to retry its whole bring-up in a loop with a handful of
//! yields between tries and a ten-second deadline. On a device that fails the
//! same way every time (the MSI-X BAR refused on a modern-only virtio NIC was
//! the case found) that is a thousand claim, refuse, release rounds, each one
//! logged by the broker, on a core nobody else gets. This is the replacement:
//! a fixed number of attempts, a sleep between them that doubles up to a cap,
//! and then one clean exit the boot log can name.
//!
//! Pure on purpose, so the schedule is proven on the host.

/// Attempts at bring-up before the driver gives up on the device.
pub const BRINGUP_ATTEMPTS: u32 = 7;
/// Sleep after the first failed attempt.
pub const BRINGUP_FIRST_DELAY_MS: u64 = 100;
/// No single sleep is longer than this.
pub const BRINGUP_MAX_DELAY_MS: u64 = 3_200;

/// The device this driver serves is not on the machine. Nothing was claimed.
pub const EXIT_ABSENT: i32 = 2;
/// The device is present and every attempt to bring it up failed. Whatever an
/// attempt claimed was released before the exit.
pub const EXIT_GAVE_UP: i32 = 6;

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

/// Where a driver's start leads: `Ok` to serve what `up` brought up, or `Err`
/// with the code to exit with.
///
/// `found` is what discovery saw. A device that is not there is never tried:
/// `absent` runs once (the driver says so) and the answer is `EXIT_ABSENT` at
/// once, because the broker lists every device before the first driver starts
/// and no retry would make one appear. A device that is there gets `up`, the
/// bounded bring-up, and a failure there is `EXIT_GAVE_UP`.
pub fn decide<F, T>(
    found: Option<F>,
    absent: impl FnOnce(),
    up: impl FnOnce(F) -> Result<T, &'static str>,
) -> Result<T, i32> {
    match found {
        None => {
            absent();
            Err(EXIT_ABSENT)
        }
        Some(device) => up(device).map_err(|_| EXIT_GAVE_UP),
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

/// What a serve loop does with its receive's answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecvTurn {
    /// A message arrived: serve it.
    Serve,
    /// The receive waited and came back with nothing: go round again.
    Again,
    /// The receive failed without waiting, and will fail the same way at
    /// once next time (an inbox that is gone, a bad buffer): sleep before
    /// going round, so the loop holds no core.
    Park,
}

/// The kernel's answer to a receive whose timeout ran out with nothing.
pub const ERRNO_TIMEDOUT: i64 = -110;
/// How long a serve loop sleeps after a receive that failed at once.
pub const RECV_PARK_MS: u64 = 100;

/// The turn after a receive that returned `rc`.
pub const fn recv_turn(rc: i64) -> RecvTurn {
    if rc > 0 {
        RecvTurn::Serve
    } else if rc == 0 || rc == ERRNO_TIMEDOUT {
        RecvTurn::Again
    } else {
        RecvTurn::Park
    }
}
