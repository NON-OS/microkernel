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
//! layer to say on the log: this driver holds no Debug capability, and a
//! stick it did not take left nothing anywhere. Pure, so the host proofs
//! hold the wire the kernel's client mirrors (src/hardware/usb_msc_capsule/
//! report.rs); kernel_proofs includes this file alone, so its parts are
//! named from here.

#[path = "interface.rs"]
mod interface;
#[path = "stage.rs"]
mod stage;

pub use interface::first_interface;
pub use stage::Stage;

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

/// The bytes `encode` writes.
pub const REPORT_LEN: usize = 20;

impl Report {
    pub const fn new() -> Self {
        Self {
            stage: Stage::NoXhci,
            port: 0,
            detail: 0,
            aux: 0,
            blocks: 0,
            closed: 0,
            connected: 0,
        }
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
