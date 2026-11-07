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

//! Direct scanout's state between frames (direct_rule.rs decides, the
//! frame pacer's direct.rs acts).

/// Refused buffers and opaque presents remembered, newest over oldest.
pub const REMEMBERED: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Direct {
    /// The pid of the wire driver whose CAPS has DIRECT_SCANOUT and
    /// BAND_PLANE; None when the driver has neither, or refused for good.
    pub driver: Option<u32>,
    /// The client buffer the plane shows now, as the driver last said.
    pub plane: Option<u64>,
    refused: [u64; REMEMBERED],
    opaque: [u64; REMEMBERED],
    at: (usize, usize),
}

impl Direct {
    /// The driver as CAPS described it.
    pub fn new(driver_pid: u32, scanout: bool) -> Self {
        Direct { driver: scanout.then_some(driver_pid), ..Direct::default() }
    }

    /// The driver it serves, when it is still `pid` (a restarted driver
    /// is primed again).
    pub fn able(&self, pid: u32) -> bool {
        self.driver == Some(pid)
    }

    pub fn refuse_driver(&mut self) {
        self.driver = None;
    }

    pub fn refuse(&mut self, handle: u64) {
        if !self.refused(handle) {
            self.refused[self.at.0] = handle;
            self.at.0 = (self.at.0 + 1) % REMEMBERED;
        }
    }

    pub fn refused(&self, handle: u64) -> bool {
        handle != 0 && self.refused.contains(&handle)
    }

    /// A present of `handle` said whether every pixel is opaque.
    pub fn presented(&mut self, handle: u64, opaque: bool) {
        let known = self.opaque.iter().position(|&h| h == handle);
        match (known, opaque) {
            (Some(_), true) | (None, false) => {}
            (Some(i), false) => self.opaque[i] = 0,
            (None, true) => {
                self.opaque[self.at.1] = handle;
                self.at.1 = (self.at.1 + 1) % REMEMBERED;
            }
        }
    }

    pub fn opaque(&self, handle: u64) -> bool {
        handle != 0 && self.opaque.contains(&handle)
    }
}
