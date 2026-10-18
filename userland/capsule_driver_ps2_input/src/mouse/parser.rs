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
use super::packet::parse;
use super::ring::MouseRing;

const MAX_PACKET: usize = 4;
/// A part packet older than this is thrown away, as Linux psmouse_interrupt
/// does. A packet's bytes come a millisecond apart, but they are stamped when
/// read, so a capsule that waited for the processor keeps a good packet.
const RESYNC_MS: u64 = 500;
/// A mouse that reset (KVM switch, re-plug, EC reset) sends 0xAA, then 0x00.
const BAT_OK: u8 = 0xAA;
const MOUSE_ID: u8 = 0x00;

pub struct MouseParser {
    buf: [u8; MAX_PACKET],
    index: usize,
    // 3 for a plain mouse, 4 once IntelliMouse scroll is negotiated.
    packet_len: usize,
    /// When the last byte was read, in uptime milliseconds.
    last_ms: u64,
    /// The mouse reset and has to be brought up again.
    reset: bool,
}

impl MouseParser {
    pub const fn new(wheel: bool) -> Self {
        let packet_len = if wheel { 4 } else { 3 };
        Self { buf: [0; MAX_PACKET], index: 0, packet_len, last_ms: 0, reset: false }
    }

    // Assemble one packet and queue it. The pump loop posts what it drains
    // from the ring; posting here as well would post every event twice.
    pub fn absorb(&mut self, byte: u8, now_ms: u64, ring: &mut MouseRing) {
        if self.index > 0 && now_ms.saturating_sub(self.last_ms) > RESYNC_MS {
            (self.index, ring.sync_errors) = (0, ring.sync_errors.wrapping_add(1));
        }
        self.last_ms = now_ms;
        if self.index == 1 && self.buf[0] == BAT_OK && byte == MOUSE_ID {
            (self.index, self.reset) = (0, true);
            return;
        }
        if self.index == 0 && byte & 0x08 == 0 {
            ring.sync_errors = ring.sync_errors.wrapping_add(1);
            return;
        }
        self.buf[self.index] = byte;
        self.index += 1;
        if self.index == self.packet_len {
            self.index = 0;
            match parse(&self.buf[..self.packet_len]) {
                Some(ev) => ring.push(ev),
                None => ring.sync_errors = ring.sync_errors.wrapping_add(1),
            }
        }
    }

    /// Whether the mouse has reset since the last time this was asked.
    pub fn take_reset(&mut self) -> bool {
        core::mem::take(&mut self.reset)
    }
}
