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

//! Whether the bound NIC's driver serves receive batches.

use core::sync::atomic::{AtomicU32, Ordering};

use super::batch_call::call;
use super::rx_batch::forget_queue;

/// The port whose driver serves batches, or 0 for none.
static BATCH_PORT: AtomicU32 = AtomicU32::new(0);
/// The port last asked.
static PROBED_PORT: AtomicU32 = AtomicU32::new(0);

/// Whether the driver behind `port` serves batches, asking once per port:
/// a newly bound NIC is asked on its first poll, and what the last one left
/// queued is dropped with it.
pub fn serves(port: u32) -> bool {
    if port != 0 && PROBED_PORT.swap(port, Ordering::AcqRel) != port {
        forget_queue();
        let served = call(port, None).is_some();
        BATCH_PORT.store(if served { port } else { 0 }, Ordering::Release);
    }
    port != 0 && BATCH_PORT.load(Ordering::Acquire) == port
}
