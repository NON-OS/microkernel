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
use core::ptr::write_volatile;

use crate::audio::PcmQueue;
use crate::controller::codec::format::PLAYBACK;
use crate::controller::codec::program::set_outputs;
use crate::controller::dma_sync::flush;
use crate::controller::{stream_run, StreamDescriptor, StreamRun};
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle_stream_start(driver: &mut Driver, req: &Request, tx: &mut [u8], running: &mut bool) {
    if !*running {
        silence(driver);
        if !restart(driver) || !open(driver, true) {
            stream_run::stop(driver.regs, driver.stream_off);
            let _ = open(driver, false);
            reply_with_status(tx, req, E_INVAL);
            return;
        }
        *running = true;
    }
    reply_with_status(tx, req, E_OK);
}

pub fn handle_stream_stop(
    driver: &mut Driver,
    req: &Request,
    tx: &mut [u8],
    q: &mut PcmQueue,
    running: &mut bool,
) {
    if *running {
        stream_run::stop(driver.regs, driver.stream_off);
        let _ = open(driver, false);
        q.clear();
        *running = false;
    }
    reply_with_status(tx, req, E_OK);
}

/// Open the codec's output amps for a stream that now runs, or close them
/// behind one that stopped. Outputs are closed whenever nothing plays, so a
/// stopped engine or a stale ring can never be heard.
pub(super) fn open(driver: &mut Driver, on: bool) -> bool {
    set_outputs(&mut driver.link, &driver.codec, &driver.plan, on).is_ok()
}

fn silence(driver: &Driver) {
    let dst = driver.sample.user_va as *mut u8;
    let cap = driver.sample.length as usize;
    let mut i = 0usize;
    while i < cap {
        unsafe { write_volatile(dst.add(i), 0u8) };
        i += 1;
    }
    flush(driver.sample.user_va, driver.sample.length);
}

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
