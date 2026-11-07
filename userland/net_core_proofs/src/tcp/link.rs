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
    /// Drop every TCP segment to this port: a peer that sent a SYN and
    /// never hears the SYN-ACK.
    pub drop_to_port: Option<u16>,
}

fn tcp_dst_port(frame: &[u8]) -> Option<u16> {
    let ihl = usize::from(frame.first()? & 0x0F) * 4;
    if frame.get(9) != Some(&6) {
        return None;
    }
    let at = ihl.checked_add(2)?;
    Some(u16::from_be_bytes([*frame.get(at)?, *frame.get(at + 1)?]))
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
        let dropped = self.0.drop_to_port.is_some_and(|p| tcp_dst_port(&frame) == Some(p));
        if !self.0.cut && !dropped {
            self.0.queue.push_back(frame);
        }
        r
    }
}
