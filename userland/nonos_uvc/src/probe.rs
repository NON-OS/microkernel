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
//! What the hub class needs of a USB host controller driver and of the
//! The video probe and commit controls (VS_PROBE_CONTROL 01h,
//! VS_COMMIT_CONTROL 02h): the 34 bytes of UVC 1.1 and later, a class
//! request to the streaming interface (wValue the selector in its high
//! byte, wIndex the interface). The host sets a probe, reads back what
//! the device will do, then commits it.

pub const SET_CUR: u8 = 0x01;
pub const GET_CUR: u8 = 0x81;
pub const VS_PROBE: u8 = 0x01;
pub const VS_COMMIT: u8 = 0x02;
pub const CONTROL_LEN_11: usize = 34;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Probe {
    /// bmHint bit 0: keep dwFrameInterval.
    pub hint: u16,
    pub format: u8,
    pub frame: u8,
    pub interval: u32,
    pub max_frame: u32,
    pub max_payload: u32,
}

impl Probe {
    pub fn bytes(&self) -> [u8; CONTROL_LEN_11] {
        let mut b = [0u8; CONTROL_LEN_11];
        b[0..2].copy_from_slice(&self.hint.to_le_bytes());
        (b[2], b[3]) = (self.format, self.frame);
        b[4..8].copy_from_slice(&self.interval.to_le_bytes());
        b[18..22].copy_from_slice(&self.max_frame.to_le_bytes());
        b[22..26].copy_from_slice(&self.max_payload.to_le_bytes());
        b
    }

    /// What a device answered (26 bytes at least, UVC 1.0's), or None.
    pub fn read(b: &[u8]) -> Option<Self> {
        if b.len() < 26 {
            return None;
        }
        let u32_at = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        Some(Probe {
            hint: u16::from_le_bytes([b[0], b[1]]),
            format: b[2],
            frame: b[3],
            interval: u32_at(4),
            max_frame: u32_at(18),
            max_payload: u32_at(22),
        })
    }
}
