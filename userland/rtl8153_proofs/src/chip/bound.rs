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

//! A model chip found and bound, for the tests that start from a bound
//! device; the calls of the bring-up are cleared, so a test sees its own.

use nonos_usbnet::found::{fetch, Found};
use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::Bind;

use super::answer::{rtl8153, Shared};
use crate::r8153::{bind, Rtl8153};

pub const RTL8153_IDS: (u16, u16) = (0x0bda, 0x8153);

/// The model on the bus and what the search would have read of it.
pub fn found(version: u16, ids: (u16, u16)) -> (MockBus, Shared, Found) {
    let (bus, chip) = rtl8153(version, ids);
    let found = fetch(&mut bus.clone(), 1).expect("descriptors");
    bus.0.borrow_mut().calls.clear();
    (bus, chip, found)
}

pub fn bound(version: u16) -> (Rtl8153<MockBus>, MockBus, Shared) {
    let (bus, chip, found) = found(version, RTL8153_IDS);
    let Bind::Ours(nic) = bind(bus.clone(), &found) else { panic!("not bound") };
    bus.0.borrow_mut().calls.clear();
    (nic, bus, chip)
}

/// How many register reads of `addr` the driver made.
pub fn reads_of(bus: &MockBus, addr: u16) -> usize {
    let calls = bus.0.borrow().calls.clone();
    calls
        .iter()
        .filter(|c| matches!(c, Call::In(s, _) if s.request == 5 && s.value == addr))
        .count()
}
