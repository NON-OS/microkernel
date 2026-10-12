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

use nonos_libc::mk_uptime_ms;

use crate::orchestrator::enumerate::{scan_hubs, Devices, RootPorts};
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::pump_once;
use crate::state::State;

use super::constants::HID_REPORT_MAX;
use super::drain_endpoints::drain_endpoints;
use super::pace::inbox_wait_ms;
use super::refresh_endpoints::refresh_endpoints;
use super::rescan::Rescan;

pub fn run(xhci_port: u32) -> ! {
    let (mut roots, mut devs) = (RootPorts::new(), Devices::new());
    let mut hubs_busy = refresh_endpoints(xhci_port, &mut devs, &mut roots);
    let mut ports = Rescan::new(now_ms());
    let mut state = State::new();
    let mut buf = [0u8; HID_REPORT_MAX];
    let mut rx = alloc::vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = alloc::vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut announced = 0usize;
    loop {
        if devs.eps.len() > announced {
            const MSG: &[u8] = b"[USB-HID-ENUM] HID device bound\n";
            let _ = nonos_libc::mk_debug(MSG.as_ptr(), MSG.len());
        }
        announced = devs.eps.len();
        let now = now_ms();
        if drain_endpoints(&mut state, &mut devs.eps, &mut buf, now) {
            ports.heard(now);
        }
        // A held key repeats from here: the keyboard reports only changes.
        devs.eps.iter_mut().for_each(|ep| ep.dev.tick(now));
        // Every port is looked at again while the devices are quiet, not only
        // until the first one binds: a keyboard or mouse plugged in after
        // another is bound, or pulled out and plugged back, was otherwise
        // never bound.
        if ports.rescan(now) {
            hubs_busy = refresh_endpoints(xhci_port, &mut devs, &mut roots);
        } else if hubs_busy && ports.hub_look(now) {
            // A hub port mid bring-up takes its next step between reads.
            hubs_busy = scan_hubs(xhci_port, &mut devs, true);
        }
        // Asleep on the inbox until the first endpoint is due to be asked.
        let wait = inbox_wait_ms(now_ms(), devs.eps.iter().map(|ep| &ep.pace));
        let _ = pump_once(&mut state, &mut rx, &mut tx, wait);
    }
}

fn now_ms() -> u64 {
    mk_uptime_ms().max(0) as u64
}
