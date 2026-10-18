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

//! Which display driver the compositor draws through: virtio-gpu when it
//! has announced itself, else the Bochs/QEMU adapter's driver, which speaks
//! the same wire. Neither leaves the compositor on GOP (wait_for_setup), and
//! the GOP-mode probe asks again in the same order (upgrade).

use core::sync::atomic::{AtomicUsize, Ordering};

pub const GFX_SERVICES: [&[u8]; 2] = [b"driver.virtio_gpu0", b"driver.bga0"];
/// What the display log calls the driver behind each of `GFX_SERVICES`.
pub const DRIVERS: [&str; 2] = ["virtio-gpu", "bga"];

/// The first of `GFX_SERVICES` that `lookup` finds announced, as its place
/// there and its port. One that is found ends the search: a later name is
/// not asked for.
pub fn first_announced(mut lookup: impl FnMut(&[u8]) -> Option<u32>) -> Option<(usize, u32)> {
    GFX_SERVICES.iter().enumerate().find_map(|(at, name)| Some((at, lookup(name)?)))
}

/// The driver to name for a lookup that found the service at `found`, or
/// `None`: when `said` names it already, and when the lookup found none,
/// as the compositor may still be waiting for one; it is on GOP only once
/// it takes the GOP framebuffer, and wait_for_setup says so then. The
/// lookup runs every few milliseconds while the compositor waits and while
/// it is on GOP, so the log gets one line for each driver found anew.
pub fn news(said: &AtomicUsize, found: Option<usize>) -> Option<&'static str> {
    let now = found.unwrap_or(GFX_SERVICES.len());
    if said.swap(now, Ordering::Relaxed) == now {
        return None;
    }
    DRIVERS.get(now).copied()
}
