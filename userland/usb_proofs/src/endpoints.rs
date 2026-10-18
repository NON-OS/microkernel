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

//! Bound endpoints as the driver makes them, each with the state of its own
//! interface: a boot keyboard, a boot mouse, or QEMU's tablet read through
//! its report descriptor; each asked every millisecond.

use crate::descriptors::report::parse;
use crate::descriptors::HidKind;
use crate::enumerate::HidEndpoint;
use crate::hid::{Device, Keyboard, Mouse, ReportDevice};
use crate::poll::Pace;
use crate::report::vectors::QEMU_TABLET;

pub fn ep(root_port: u8, slot: u8, dci: u8, kind: HidKind) -> HidEndpoint {
    let dev = match kind {
        HidKind::Keyboard => Device::Keyboard(Keyboard::new()),
        HidKind::Mouse => Device::Mouse(Mouse::new()),
        HidKind::Report => {
            let layout = parse(QEMU_TABLET).expect("QEMU's tablet");
            Device::Report(Box::new(ReportDevice::new(layout)))
        }
    };
    HidEndpoint { port: 7, root_port, slot, dci, kind, max_packet: 8, dev, pace: Pace::new(1) }
}

/// Every event posted since the last take, as (kind, code).
pub fn posted() -> Vec<(u16, u32)> {
    nonos_libc::take_posted().iter().map(|e| (e.kind, e.code)).collect()
}
