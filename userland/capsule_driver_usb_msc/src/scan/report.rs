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


//! Where the search for the device stands, and why, for the kernel's block
//! layer to say on the log. This driver holds no Debug capability and says
//! nothing itself: a stick it did not take left nothing anywhere, and a boot
//! from it showed only that the store was not there. Pure, so the host
//! proofs hold the wire, which the kernel's client mirrors
//! (src/hardware/usb_msc_capsule/report.rs).

/// What the search last did. The numbers are the wire's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Stage {
    /// driver.xhci0 has not registered yet.
    NoXhci = 1,
    /// The controller answers and no port has a device connected.
    NoPort = 2,
    /// Another class driver is addressing the port.
    Busy = 3,
    /// No device slot, or the device would not take an address.
    Address = 4,
    /// Its configuration descriptor did not read or did not parse.
    Config = 5,
    /// No SCSI-transparent Bulk-Only interface; `aux` is the first
    /// interface's class, subclass and protocol.
    NotStorage = 6,
    /// SET_CONFIGURATION or the bulk pipes were refused.
    Configure = 7,
    /// INQUIRY or TEST UNIT READY did not succeed in five seconds.
    NotReady = 8,
    /// READ CAPACITY(10) failed.
    Capacity = 9,
    /// Bound: `blocks` of `aux` bytes.
    Bound = 10,
}

/// The last stage, the port it was on, an errno, and what it read.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Report {
    pub stage: Stage,
    pub port: u8,
    pub detail: i32,
    pub aux: u32,
    pub blocks: u64,
    /// Ports given up on, and ports connected, at the last pass.
    pub closed: u8,
    pub connected: u8,
}

/// An interface descriptor's type, as descriptors/wire.rs names it.
const DESC_INTERFACE: u8 = 0x04;

/// The bytes `encode` writes.
pub const REPORT_LEN: usize = 20;

impl Report {
    pub const fn new() -> Self {
        Self { stage: Stage::NoXhci, port: 0, detail: 0, aux: 0, blocks: 0, closed: 0, connected: 0 }
    }

    pub fn at(stage: Stage, port: u8, detail: i32) -> Self {
        Self { stage, port, detail, ..Self::new() }
    }

    pub fn encode(&self, out: &mut [u8]) -> usize {
        out[0] = self.stage as u8;
        out[1] = self.port;
        out[2] = self.closed;
        out[3] = self.connected;
        out[4..8].copy_from_slice(&self.detail.to_le_bytes());
        out[8..12].copy_from_slice(&self.aux.to_le_bytes());
        out[12..20].copy_from_slice(&self.blocks.to_le_bytes());
        REPORT_LEN
    }
}

/// The first interface's class, subclass and protocol in a configuration
/// descriptor, as `0x00CCSSPP`, or 0 when it has none.
pub fn first_interface(raw: &[u8]) -> u32 {
    let mut pos = 0;
    while pos + 2 <= raw.len() {
        let len = raw[pos] as usize;
        if len < 2 || pos + len > raw.len() {
            return 0;
        }
        if raw[pos + 1] == DESC_INTERFACE && len >= 9 {
            return u32::from_be_bytes([0, raw[pos + 5], raw[pos + 6], raw[pos + 7]]);
        }
        pos += len;
    }
    0
}
