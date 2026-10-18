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

//! An interrupt endpoint's payload per service interval, and the Endpoint
//! Context fields it fills (xHCI 1.2 section 6.2.3): Max Burst Size, Max
//! ESIT Payload and Average TRB Length. A high-bandwidth high-speed
//! endpoint (wMaxPacketSize bits 12:11) or a bursting SuperSpeed one moves
//! up to three packets, and was given one.

use crate::contexts::payload::Payload;
use crate::contexts::{interrupt_payload, write_configure_endpoint_input, EndpointConfig};
use crate::dma::DmaPool;

const FULL: u8 = 1;
const HIGH: u8 = 3;
const SUPER: u8 = 4;

fn payload(max_packet: u16, max_burst: u8, esit: u16) -> Option<Payload> {
    Some(Payload { max_packet, max_burst, esit })
}

#[test]
fn every_packet_of_the_interval_is_counted_and_nothing_past_the_specification() {
    assert_eq!(interrupt_payload(HIGH, false, 1024 | 2 << 11, 0), payload(1024, 2, 3072));
    assert_eq!(interrupt_payload(HIGH, false, 512 | 1 << 11, 9), payload(512, 1, 1024));
    assert_eq!(interrupt_payload(HIGH, false, 64 | 3 << 11, 0), None, "mult 3 is reserved");
    assert_eq!(interrupt_payload(FULL, false, 64 | 2 << 11, 2), payload(64, 0, 64));
    assert_eq!(interrupt_payload(SUPER, true, 1024 | 3 << 11, 2), payload(1024, 2, 3072));
    assert_eq!(interrupt_payload(SUPER, true, 1024, 3), None, "three packets at most");
    assert_eq!(interrupt_payload(SUPER, true, 1024, 0xFF), None);
    for wild in [0, 1025, 0x7FF, 0xFFFF] {
        assert_eq!(interrupt_payload(HIGH, false, wild & !(3 << 11), 0), None, "{wild:#x}");
    }
}

#[test]
fn the_endpoint_context_carries_the_burst_and_the_whole_payload() {
    let pool = DmaPool::new(9, 1);
    let (input, output) = (pool.alloc(4096).expect("input"), pool.alloc(4096).expect("output"));
    let cfg = EndpointConfig {
        context_size: 32,
        dci: 3,
        ring_phys: 0x1_0000,
        payload: Payload { max_packet: 1024, max_burst: 2, esit: 3072 },
        interval: 3,
    };
    write_configure_endpoint_input(&input, &output, cfg);
    let dw = |n: usize| input.read_u32(4 * 32 + 4 * n).expect("in the input context");
    assert_eq!(dw(0) >> 8 & 0x3, 0, "Mult: 0 for an interrupt endpoint");
    assert_eq!(dw(0) >> 24, 0, "Max ESIT Payload Hi");
    assert_eq!(dw(1) >> 8 & 0xFF, 2, "Max Burst Size");
    assert_eq!(dw(1) >> 16, 1024, "Max Packet Size");
    assert_eq!(dw(4) & 0xFFFF, 3072, "Average TRB Length");
    assert_eq!(dw(4) >> 16, 3072, "Max ESIT Payload Lo");
}
