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
//! nonos_usb_hub's Host over one USB host's wire, with the kernel's clock
//! and debug log.

use nonos_libc::{mk_debug, mk_idle_ms, mk_uptime_ms};
use nonos_usb_hub::{park_body, Host, Place, RootHost, Setup, StatusEndpoint, OP_PARK};

use crate::control::control;
use crate::wire::*;

pub use crate::link::Wire;

impl Host for Wire {
    fn control(&mut self, slot: u8, s: Setup, data: &mut [u8]) -> Result<usize, i32> {
        control(self, slot, s, data)
    }
    fn configure_hub(&mut self, slot: u8, ports: u8, think: u8) -> Result<(), i32> {
        self.op(OP_CONFIGURE_HUB, &[slot, ports, think, 0], &mut []).map(|_| ())
    }
    fn status_endpoint(&mut self, slot: u8, ep: StatusEndpoint) -> Result<(), i32> {
        let [m0, m1] = ep.max_packet.to_le_bytes();
        let body = [slot, ep.address, m0, m1, ep.interval, 0];
        self.op(OP_ALLOC_TRANSFER_RING, &body, &mut [0; 4]).map(|_| ())
    }
    fn enable_slot(&mut self) -> Result<u8, i32> {
        let mut back = [0u8; 4];
        self.op(OP_ENABLE_SLOT, &[], &mut back).map(|_| back[0])
    }
    fn address_routed(&mut self, slot: u8, p: &Place) -> Result<(), i32> {
        let [r0, r1, r2, r3] = p.route.to_le_bytes();
        let (ts, tp) = p.tt.map_or((0, 0), |t| (t.hub_slot, t.port));
        let body = [slot, p.root_port, p.speed, r0, r1, r2, r3, ts, tp];
        self.op(OP_ADDRESS_ROUTED, &body, &mut [0; 8]).map(|_| ())
    }
    fn disable_slot(&mut self, slot: u8) {
        let _ = self.op(OP_DISABLE_SLOT, &[slot], &mut []);
    }
    fn park(&mut self, slot: u8, place: &Place, mps0: u16) -> Result<(), i32> {
        self.op(OP_PARK, &park_body(slot, place, mps0), &mut []).map(|_| ())
    }
    fn idle_ms(&mut self, ms: u64) {
        let _ = mk_idle_ms(ms);
    }
    fn now_ms(&mut self) -> i64 {
        mk_uptime_ms()
    }
    fn say(&mut self, line: &[u8]) {
        let _ = mk_debug(line.as_ptr(), line.len());
    }
}

impl RootHost for Wire {
    fn address_root(&mut self, slot: u8, port: u8) -> Result<u8, i32> {
        let mut back = [0u8; 8];
        self.op(OP_ADDRESS_DEVICE, &[slot, port], &mut back).map(|_| back[2])
    }
}
