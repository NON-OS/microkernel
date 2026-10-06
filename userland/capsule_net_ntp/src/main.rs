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

#![no_std]
#![no_main]

extern crate alloc;

mod decide;
mod setup;
mod sntp;
mod state;
mod sync;
mod udp_client;
mod util;

use alloc::format;

use decide::{skip_line, step, Step};
use nonos_libc::{heap_init, mk_exit, mk_yield};
use sync::sync_once;
use util::{log, sleep_ms};

const RESYNC_INTERVAL_MS: u64 = 900_000;

/* While the chosen network is anonymous, how often it is looked at again:
 * the answers setup kept are restored into the policy store after this
 * capsule starts, and Settings can change the choice at any time, so a
 * machine set to Direct is synced soon after the store says so. */
const ROUTE_CHECK_MS: u64 = 30_000;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let mut said = None;
    let mut bound = false;
    loop {
        match step(nonos_route_link::direct_refusal(), said) {
            Step::Skip { why, log: news } => {
                if news {
                    log(&skip_line(why));
                }
                said = Some(why);
                sleep_ms(ROUTE_CHECK_MS);
            }
            Step::Sync => {
                said = None;
                /* The UDP port is bound only once a time server may be
                 * asked: nothing of this capsule's touches the network on
                 * a machine set to an anonymous one. */
                if !bound {
                    wait_for_setup();
                    bound = true;
                }
                match sync_once() {
                    Some(ms) => log(&format!("[NTP] SYNC ok unix_ms={}\n", ms)),
                    None => log("[NTP] no-sync\n"),
                }
                sleep_ms(RESYNC_INTERVAL_MS);
            }
        }
    }
}

fn wait_for_setup() {
    loop {
        if setup::run().is_ok() {
            return;
        }
        for _ in 0..64 {
            mk_yield();
        }
    }
}
