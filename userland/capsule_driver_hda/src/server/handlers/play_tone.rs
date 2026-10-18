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

use super::restart::restart;
use super::stream::open;
use crate::audio;
use crate::controller::dma_sync::flush;
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

/// One pass of the test tone on request. The stream counts as running, so
/// the refill overwrites the ring with whatever the queue holds (silence when
/// it is empty) after the first pass, and OP_STREAM_STOP ends it and closes
/// the outputs. A tone left looping on its own never stops.
pub fn handle(
    driver: &mut Driver,
    req: &Request,
    tx: &mut [u8],
    played: &mut bool,
    running: &mut bool,
) {
    let (va, len) = (driver.sample.user_va, driver.sample.length);
    // SAFETY: the sample grant is `len` bytes the broker mapped at `va` for
    // this capsule alone, and no other reference into it is alive here.
    audio::fill(unsafe { core::slice::from_raw_parts_mut(va as *mut u8, len as usize) });
    flush(va, len);
    let ok = restart(driver) && open(driver, true);
    if !ok {
        crate::controller::stream_run::stop(driver.regs, driver.stream_off);
        let _ = open(driver, false);
    }
    *played = false;
    *running = ok;
    reply_with_status(tx, req, if ok { E_OK } else { E_INVAL });
}
