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

//! A Wi-Fi driver keeps the frame it handed net.core until net.core asks the
//! next number, so a reply that came too late is not lost. Its own binary:
//! the kept frame is the adapter's one static.

use nonos_wifi_core::netif::serve::{serve, LinkPort, MAX_RESPONSE};
use nonos_wifi_core::netif::wire::{HDR_LEN, MAGIC_NNET, OP_RX_PACKET, VERSION};

struct Queue(Vec<u8>);

impl LinkPort for Queue {
    fn mac(&self) -> Option<[u8; 6]> {
        Some([2, 0, 0, 0, 0, 1])
    }
    fn link_up(&self) -> bool {
        true
    }
    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize> {
        (!self.0.is_empty()).then(|| (out[0] = self.0.remove(0), 1).1)
    }
    fn send_tx(&mut self, _: &[u8]) -> bool {
        true
    }
}

/// An OP_RX_PACKET for `seq` (none: the old wire), and the frame's byte.
fn rx(link: &mut Queue, seq: Option<u32>) -> Option<u8> {
    let body = seq.map(u32::to_le_bytes);
    let body: &[u8] = body.as_ref().map_or(&[], |b| b);
    let mut req = vec![0u8; HDR_LEN + body.len()];
    req[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&OP_RX_PACKET.to_le_bytes());
    req[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    req[HDR_LEN..].copy_from_slice(body);
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&req, link, &mut out)?;
    let status = i32::from_le_bytes(out[HDR_LEN..HDR_LEN + 4].try_into().ok()?);
    (status == 0 && n > HDR_LEN + 8).then(|| out[HDR_LEN + 8])
}

#[test]
fn a_reask_gets_the_same_frame_the_next_number_the_next_and_the_old_wire_a_fresh_one() {
    let mut link = Queue(vec![1, 2, 3]);
    assert_eq!(rx(&mut link, Some(5)), Some(1));
    assert_eq!(rx(&mut link, Some(5)), Some(1), "the late reply's frame again");
    assert_eq!(rx(&mut link, Some(6)), Some(2));
    assert_eq!(rx(&mut link, None), Some(3), "no body: a fresh frame, as before");
    assert_eq!(rx(&mut link, None), None);
}
