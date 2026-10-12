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

//! The isochronous pieces that need no controller: the burst fields of a TD
//! (xHCI 1.2 4.11.2.3) and the Interval (6.2.3.6), both as recalled and
//! unverified; the endpoint a descriptor becomes at each speed.

use crate::contexts::{isoch_interval, SPEED_HIGH};

const SPEED_FULL: u8 = 1;
use crate::controller::{burst_fields, isoch_endpoint};

#[test]
fn below_superspeed_a_td_counts_packets_not_bursts() {
    assert_eq!(burst_fields(192, 192, 0, false), (0, 0), "one full-speed packet");
    assert_eq!(burst_fields(3072, 1024, 2, false), (0, 2), "three high-bandwidth packets");
    assert_eq!(burst_fields(1, 1024, 2, false), (0, 0));
}

#[test]
fn at_superspeed_a_td_counts_bursts_and_the_last_ones_packets() {
    // Two packets a burst: four packets are two bursts, the last one full.
    assert_eq!(burst_fields(4096, 1024, 1, true), (1, 1));
    // Three packets: two bursts, one packet in the last.
    assert_eq!(burst_fields(3072, 1024, 1, true), (1, 0));
    assert_eq!(burst_fields(1024, 1024, 0, true), (0, 0));
    assert_eq!(burst_fields(2048, 1024, 3, true), (0, 1));
}

#[test]
fn the_interval_is_an_exponent_of_microframes_at_every_speed() {
    assert_eq!(isoch_interval(SPEED_FULL, false, 1), 3, "every 1 ms frame");
    assert_eq!(isoch_interval(SPEED_FULL, false, 4), 6, "every 8 ms");
    assert_eq!(isoch_interval(SPEED_HIGH, false, 1), 0, "every microframe");
    assert_eq!(isoch_interval(SPEED_HIGH, false, 4), 3, "every 1 ms");
    assert_eq!(isoch_interval(SPEED_FULL, true, 4), 3, "a USB 3 port is SuperSpeed");
    assert_eq!(isoch_interval(SPEED_HIGH, false, 0), 0, "a zero bInterval is taken as 1");
    assert_eq!(isoch_interval(SPEED_FULL, false, 16), 15, "never past the field");
}

#[test]
fn the_endpoint_takes_bursts_from_where_each_speed_keeps_them() {
    let fs = isoch_endpoint(SPEED_FULL, false, 4, 1, (5, 2), 0x00C0);
    assert_eq!((fs.max_packet, fs.max_burst, fs.mult, fs.esit_payload()), (192, 0, 0, 192));
    // High bandwidth: 1024 bytes and two more transactions in bits 12:11.
    let hs = isoch_endpoint(SPEED_HIGH, false, 4, 1, (5, 2), 0x1400);
    assert_eq!((hs.max_packet, hs.max_burst, hs.mult, hs.esit_payload()), (1024, 2, 0, 3072));
    let ss = isoch_endpoint(SPEED_HIGH, true, 4, 1, (3, 1), 0x0400);
    assert_eq!((ss.max_packet, ss.max_burst, ss.mult, ss.esit_payload()), (1024, 3, 1, 8192));
}
