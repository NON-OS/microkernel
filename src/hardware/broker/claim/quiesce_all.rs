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

extern crate alloc;

use super::state::CLAIMS;

/// Stop every claimed device from mastering the bus, for the shutdown wipe:
/// a device still writing would put bytes back behind it. The claim table is
/// waited for a millisecond at most, since a CPU stopped by IPI may hold it
/// (`brief`); the claims themselves are left in place. The number of devices
/// stopped, or `None` if the table stayed held.
pub fn quiesce_all() -> Option<usize> {
    let start = crate::time::now_ns();
    let held: alloc::vec::Vec<u64> = {
        let claims = loop {
            if let Some(claims) = CLAIMS.try_lock() {
                break claims;
            }
            if !super::super::brief::within_a_millisecond(start) {
                return None;
            }
            // A holder may be in a shootdown round waiting on this CPU (L75).
            crate::smp::serve_shootdowns();
            core::hint::spin_loop();
        };
        claims.iter().map(|c| c.device_id).collect()
    };
    for device_id in &held {
        super::quiesce::stop_bus_master_quietly(*device_id);
    }
    Some(held.len())
}
