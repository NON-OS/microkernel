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
//! nonos_usb_hub's Host over the host wire this driver speaks, with the
//! kernel's clock and debug log.

use nonos_libc::{mk_debug, mk_idle_ms, mk_uptime_ms};
use nonos_usb_hub::{Host, Place, Setup, StatusEndpoint};

use crate::xhci::{
    address_routed, alloc_transfer_ring, configure_hub, control_transfer, disable_slot, enable_slot,
};

use super::status::status;

pub struct XhciHost {
    pub port: u32,
}

impl Host for XhciHost {
    fn control(&mut self, slot: u8, s: Setup, data: &mut [u8]) -> Result<usize, i32> {
        let (rt, rq, v, i, n) = (s.request_type, s.request, s.value, s.index, s.length);
        control_transfer(self.port, slot, rt, rq, v, i, n, data).map_err(status)
    }

    fn configure_hub(&mut self, slot: u8, ports: u8, think: u8) -> Result<(), i32> {
        configure_hub(self.port, slot, ports, think).map_err(status)
    }

    fn status_endpoint(&mut self, slot: u8, ep: StatusEndpoint) -> Result<(), i32> {
        alloc_transfer_ring(self.port, slot, ep.address, ep.max_packet, ep.interval)
            .map(|_| ())
            .map_err(status)
    }

    fn enable_slot(&mut self) -> Result<u8, i32> {
        enable_slot(self.port).map_err(status)
    }

    fn address_routed(&mut self, slot: u8, place: &Place) -> Result<(), i32> {
        address_routed(self.port, slot, place).map(|_| ()).map_err(status)
    }

    fn disable_slot(&mut self, slot: u8) {
        disable_slot(self.port, slot)
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
