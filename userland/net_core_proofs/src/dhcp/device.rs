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

//! The link the DHCP proofs run net.core's client on.

use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::time::Instant;

/// A link that keeps every frame sent, in order, and delivers none.
#[derive(Default)]
pub(super) struct Wire(pub(super) Vec<Vec<u8>>);

pub(super) struct Rx;
pub(super) struct Tx<'a>(&'a mut Wire);

impl RxToken for Rx {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, f: F) -> R {
        f(&mut [])
    }
}

impl TxToken for Tx<'_> {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, len: usize, f: F) -> R {
        let mut frame = vec![0u8; len];
        let r = f(&mut frame);
        self.0 .0.push(frame);
        r
    }
}

impl Device for Wire {
    type RxToken<'a> = Rx;
    type TxToken<'a> = Tx<'a>;
    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = 1514;
        caps
    }
    fn receive(&mut self, _now: Instant) -> Option<(Rx, Tx<'_>)> {
        None
    }
    fn transmit(&mut self, _now: Instant) -> Option<Tx<'_>> {
        Some(Tx(self))
    }
}
