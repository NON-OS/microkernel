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
//! Who may see the camera. The kernel never sees a camera, so this driver
//! is the gate: a peer is served only while the kernel says it holds
//! CameraCapture (MkCapCheck), asked when it opens the camera and again
//! whenever it starts a stream, and only the peer that started a stream
//! reads it. Pure but for the one question, which the caller answers.

use crate::wire::{E_BUSY, E_PERM};

/// CameraCapture, bit 37 of a capability mask (the the maintainers' branch, as
/// nonos_cap's CAP_CAMERA_CAPTURE and the manifest's "CameraCapture").
pub const CAMERA_CAPTURE: u64 = 1 << 37;

#[derive(Default)]
pub struct Session {
    /// The peer the stream runs for.
    pub streamer: Option<u32>,
    /// Whole frames assembled since the stream started.
    pub sequence: u32,
}

impl Session {
    /// OP_OPEN by `peer`, whose holding of CameraCapture is `holds`.
    pub fn open(&self, holds: bool) -> Result<(), i32> {
        if holds {
            Ok(())
        } else {
            Err(E_PERM)
        }
    }

    /// OP_START by `peer`: asked again, and refused while another peer's
    /// stream runs.
    pub fn start(&mut self, peer: u32, holds: bool) -> Result<(), i32> {
        if !holds {
            return Err(E_PERM);
        }
        match self.streamer {
            Some(other) if other != peer => Err(E_BUSY),
            _ => {
                (self.streamer, self.sequence) = (Some(peer), 0);
                Ok(())
            }
        }
    }

    /// OP_READ_FRAME and OP_STOP: the streaming peer's alone.
    pub fn own(&self, peer: u32) -> Result<(), i32> {
        if self.streamer == Some(peer) {
            Ok(())
        } else {
            Err(E_PERM)
        }
    }

    pub fn stop(&mut self) {
        self.streamer = None;
    }
}
