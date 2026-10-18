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
//! A virtio-gpu that came up: its device id, which a DGON names, the wire
//! server over its display (G-77), and what an in-place recovery of its
//! control queue needs.

use nonos_gfx_serve::{Platform, Server};
use nonos_libc::mk_device_release;
use nonos_virtio::common::{reset_until, NotifyArea};

use crate::clock::expired_after;
use crate::constants::RESET_MS;
use crate::display::Gpu;
use crate::state::Tally;

/// Generic over the primaries' memory: the shipping driver's is
/// `nonos_gfx_pool::Pool`, which asks the kernel.
pub struct Driver<P: Platform> {
    pub device_id: u64,
    pub server: Server<Gpu, P>,
    /// The doorbell geometry, to program the queue again after a reset.
    pub notify: NotifyArea,
    pub tally: Tally,
}

impl<P: Platform> Driver<P> {
    /// Stop the device and give it back: the reset drops every
    /// command and resource, then MkDeviceRelease clears bus mastering and
    /// only after that tears down the queue, surface and register grants.
    /// Only before the service is registered: from then on the compositor
    /// may hold a primary, and a reset device shows none of it.
    pub fn quiesce(&self) -> bool {
        let common = self.server.display.q.regs.common;
        let stopped = reset_until(&common, expired_after(RESET_MS)).is_ok();
        mk_device_release(self.device_id) >= 0 && stopped
    }
}
