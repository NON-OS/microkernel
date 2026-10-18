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

//! A link for the TCP proofs: every frame sent comes back to the same
//! interface, and `cut` drops every frame, as a rebooting router does.

use std::collections::VecDeque;

use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::time::Instant;

#[derive(Default)]
pub struct Link {
    queue: VecDeque<Vec<u8>>,
    pub cut: bool,
}

impl Device for Link {
    type RxToken<'a> = Rx;
    type TxToken<'a> = Tx<'a>;

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ip;
        caps.max_transmission_unit = 1500;
        caps
    }

    fn receive(&mut self, _now: Instant) -> Option<(Rx, Tx<'_>)> {
        let frame = self.queue.pop_front()?;
        Some((Rx(frame), Tx(self)))
    }

    fn transmit(&mut self, _now: Instant) -> Option<Tx<'_>> {
        Some(Tx(self))
    }
}

pub struct Rx(Vec<u8>);

impl RxToken for Rx {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(mut self, f: F) -> R {
        f(&mut self.0)
    }
}

pub struct Tx<'a>(&'a mut Link);

impl TxToken for Tx<'_> {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, len: usize, f: F) -> R {
        let mut frame = vec![0u8; len];
        let r = f(&mut frame);
        if !self.0.cut {
            self.0.queue.push_back(frame);
        }
        r
    }
}
