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

//! How the stream is cut into packets: one per service interval, each the
//! frames 48 kHz moves in it, and a batch of them per isoch request.

use super::format::{RATE_HZ, SAMPLE_BYTES};
use super::packet::per_interval;

/// Port speed ids (xHCI PORTSC bits 13:10).
pub const SPEED_HIGH: u8 = 3;

/// The most one isoch request carries: driver.xhci0's ISOCH_PACKETS_MAX
/// and ISOCH_MAX.
const BATCH_PACKETS_MAX: u32 = 31;
/// Less than ISOCH_MAX: the request also carries a u16 per packet, and the
/// xHCI client sends 4112 body bytes at most.
const BATCH_BYTES_MAX: u32 = 4032;
/// About this much sound per request, so a write the audio server makes
/// while a batch plays waits no longer than one.
const BATCH_US: u32 = 8_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Timing {
    pub packet_bytes: u16,
    pub packets: u8,
}

impl Timing {
    pub fn batch_bytes(&self) -> usize {
        self.packet_bytes as usize * self.packets as usize
    }
}

/// `None` when the endpoint cannot carry 48 kHz of `channels` at its
/// interval: a fraction of a frame per interval, more than its packets
/// hold, or a wMaxPacketSize no endpoint may have (packet.rs).
pub fn timing(speed: u8, b_interval: u8, max_packet: u16, channels: u8) -> Option<Timing> {
    let exp = b_interval.clamp(1, 16) as u32 - 1;
    // Full speed counts 1 ms frames, faster buses 125 us microframes.
    let unit_us = if speed >= SPEED_HIGH { 125 } else { 1000 };
    let interval_us = unit_us << exp.min(10);
    let frames_e6 = RATE_HZ as u64 * interval_us as u64;
    if !frames_e6.is_multiple_of(1_000_000) {
        return None;
    }
    let packet = (frames_e6 / 1_000_000) as u32 * SAMPLE_BYTES * channels as u32;
    if packet == 0 || packet > per_interval(speed, max_packet)? {
        return None;
    }
    let packets =
        (BATCH_US / interval_us).clamp(1, BATCH_PACKETS_MAX).min(BATCH_BYTES_MAX / packet);
    (packets > 0).then_some(Timing { packet_bytes: packet as u16, packets: packets as u8 })
}
