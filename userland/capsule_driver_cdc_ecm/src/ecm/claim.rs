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

//! Interfaces held with the host before the device is configured. A
//! configuration change resets every interface of the device, so it is
//! made only when this driver holds all of the configuration's interfaces;
//! one another driver holds is never taken over, and a driver that joins
//! the device later finds them held.

use alloc::vec::Vec;

use nonos_usbnet::desc::interfaces;
use nonos_usbnet::xhci::E_BUSY;

/// A bus that can hold interfaces of its device with the host: -16 when
/// another driver holds one. A host that keeps no holds answers otherwise,
/// and the device, addressed by this driver alone, is still served.
pub trait Claim {
    fn claim(&mut self, port: u8, interface: u8) -> Result<(), i32>;
}

pub fn hold<B: Claim>(bus: &mut B, port: u8, raw: &[u8]) -> Result<(), (&'static str, i32)> {
    let mut numbers: Vec<u8> = interfaces(raw).iter().map(|i| i.number).collect();
    numbers.sort_unstable();
    numbers.dedup();
    for n in numbers {
        if bus.claim(port, n) == Err(E_BUSY) {
            return Err(("interface held by another driver", E_BUSY));
        }
    }
    Ok(())
}
