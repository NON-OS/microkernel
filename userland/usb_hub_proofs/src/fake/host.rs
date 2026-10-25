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
//! nonos_usb_hub's Host over modelled hubs and a clock that moves only when
//! the walk waits: every request, slot and log line is kept for the tests.

use nonos_usb_hub::{Host, Place, Setup, StatusEndpoint};

use super::control::answer;
use super::standard::standard;
use super::state::Fake;

impl Host for Fake {
    fn control(&mut self, slot: u8, s: Setup, data: &mut [u8]) -> Result<usize, i32> {
        if (s.request_type, s.request, s.value) == (0x23, 0x03, 4) {
            self.last_reset = Some((slot, s.index as usize));
            self.resets.push((slot, s.index as usize));
        }
        if let Some(done) = self.devices.get(&slot).and_then(|d| standard(d, s, data)) {
            return done;
        }
        answer(self.hubs.get_mut(&slot).ok_or(-32)?, s, data)
    }
    fn configure_hub(&mut self, slot: u8, ports: u8, think: u8) -> Result<(), i32> {
        self.configured.push((slot, ports, think));
        self.refuse_configure.map_or(Ok(()), Err)
    }
    fn status_endpoint(&mut self, slot: u8, _ep: StatusEndpoint) -> Result<(), i32> {
        self.claimed.push(slot);
        Ok(())
    }
    fn enable_slot(&mut self) -> Result<u8, i32> {
        self.next_slot += 1;
        Ok(self.next_slot)
    }
    fn address_routed(&mut self, slot: u8, place: &Place) -> Result<(), i32> {
        if let Some(e) = self.refuse_address {
            return Err(e);
        }
        self.addressed_at(slot, place);
        Ok(())
    }
    fn park(&mut self, slot: u8, place: &Place, mps0: u16) -> Result<(), i32> {
        self.parked.push((slot, *place, mps0));
        Ok(())
    }
    fn disable_slot(&mut self, slot: u8) {
        self.disabled.push(slot);
        self.devices.remove(&slot);
    }
    fn idle_ms(&mut self, ms: u64) {
        self.now += ms as i64;
        self.slept += ms;
    }
    fn now_ms(&mut self) -> i64 {
        self.now
    }
    fn say(&mut self, line: &[u8]) {
        self.lines.push(String::from_utf8_lossy(line).into_owned());
    }
}
