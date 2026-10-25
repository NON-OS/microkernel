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

/// The device this driver serves is not on the machine. Nothing was claimed.
pub const EXIT_ABSENT: i32 = 2;
/// The device is present and every attempt to bring it up failed. Whatever an
/// attempt claimed was released before the exit.
pub const EXIT_GAVE_UP: i32 = 6;
/// A controller that came up with nothing behind it on every attempt (a
/// disk controller with no disk): a driver that can tell its give-up apart
/// leaves with this, not EXIT_GAVE_UP. Nothing it claimed is still held.
pub const EXIT_EMPTY: i32 = 3;

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
