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

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;

use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::pump_once;
use crate::state::State;
use crate::xhci::Hosts;

use super::constants::PAD_REPORT_MAX;
use super::drain_endpoints::drain_endpoints;
use super::held::Held;
use super::pace::{inbox_wait_ms, Pace};
use super::refresh_endpoints::refresh_endpoints;
use super::rescan::Rescan;

pub fn run(mut hosts: Hosts<Held>) -> ! {
    refresh_endpoints(&mut hosts);
    let mut ports = Rescan::new(now_ms());
    let mut state = State::new();
    let mut buf = [0u8; PAD_REPORT_MAX];
    let mut rx = alloc::vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = alloc::vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut announced = 0usize;
    loop {
        let mut bound = 0;
        hosts.each(|_, held| bound += held.devs.eps.len());
        if bound > announced {
            const MSG: &[u8] = b"[USB-HID-ENUM] HID device bound\n";
            let _ = nonos_libc::mk_debug(MSG.as_ptr(), MSG.len());
        }
        announced = bound;
        let now = now_ms();
        let mut drained = false;
        hosts.each(|_, held| {
            drained |= drain_endpoints(&mut state, &mut held.devs.eps, &mut buf, now)
        });
        if drained {
            ports.heard(now);
        }
        // A held key repeats from here: the keyboard reports only changes.
        hosts.each(|_, held| held.devs.eps.iter_mut().for_each(|ep| ep.dev.tick(now)));
        // Every port is looked at again while the devices are quiet, not only
        // until the first one binds: a keyboard or mouse plugged in after
        // another is bound, or pulled out and plugged back, was otherwise
        // never bound.
        if ports.rescan(now) {
            refresh_endpoints(&mut hosts);
        }
        // Asleep on the inbox until the first endpoint is due to be asked.
        let mut paces = Vec::<Pace>::new();
        hosts.each(|_, held| paces.extend(held.devs.eps.iter().map(|ep| ep.pace)));
        let wait = inbox_wait_ms(now_ms(), paces.iter());
        let _ = pump_once(&mut state, &mut rx, &mut tx, wait);
    }
}

fn now_ms() -> u64 {
    mk_uptime_ms().max(0) as u64
}
