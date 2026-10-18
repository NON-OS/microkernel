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
//! Starting the engine again from the top of the ring, for a player's
//! stream and the test tone alike.

use core::ptr::write_volatile;

use crate::controller::codec::format::PLAYBACK;
use crate::controller::dma_sync::flush;
use crate::controller::{stream_run, StreamDescriptor, StreamRun};
use crate::setup::Driver;

/// Reset the engine and start it from the top of the ring. False when the
/// descriptor did not finish its reset.
pub(super) fn restart(driver: &Driver) -> bool {
    let desc = StreamDescriptor {
        kind: driver.stream_kind,
        local_index: 0,
        global_index: driver.stream_gi,
        mmio_offset: driver.stream_off,
    };
    if let Some(va) = driver.posbuf_va {
        // SAFETY: the stream's 8-byte entry in the BDL page's position buffer
        // (`position_buffer`), aligned and inside that grant.
        unsafe { write_volatile(va as *mut u32, 0) };
        flush(va, 8);
    }
    stream_run::run(
        driver.regs,
        StreamRun {
            desc,
            tag: driver.stream_tag,
            format: PLAYBACK,
            bdl_va: driver.bdl.user_va,
            bdl_dev: driver.bdl.device_addr,
            sample_dev: driver.sample.device_addr,
            bytes: driver.sample.length as u32,
        },
    )
    .is_ok()
}
