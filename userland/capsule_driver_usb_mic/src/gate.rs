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
//! Who may hear the microphone. The kernel never sees one, so this driver
//! is the gate: a peer is served only while the kernel says it holds
//! MicrophoneCapture (MkCapCheck), asked when it opens the microphone and
//! again whenever it starts a stream, and only the peer that started a
//! stream reads it. Pure but for the one question, which the caller asks.

use crate::wire::{E_BUSY, E_PERM};

/// MicrophoneCapture, bit 38 of a capability mask (the the maintainers' ruling, as
/// the manifest's "MicrophoneCapture").
pub const MICROPHONE_CAPTURE: u64 = 1 << 38;

#[derive(Default)]
pub struct Session {
    /// The peer the stream runs for.
    pub streamer: Option<u32>,
}

impl Session {
    pub fn open(&self, holds: bool) -> Result<(), i32> {
        holds.then_some(()).ok_or(E_PERM)
    }

    /// Asked again, and refused while another peer's stream runs.
    pub fn start(&mut self, peer: u32, holds: bool) -> Result<(), i32> {
        if !holds {
            return Err(E_PERM);
        }
        match self.streamer {
            Some(other) if other != peer => Err(E_BUSY),
            _ => {
                self.streamer = Some(peer);
                Ok(())
            }
        }
    }

    /// READ and STOP: the streaming peer's alone.
    pub fn own(&self, peer: u32) -> Result<(), i32> {
        (self.streamer == Some(peer)).then_some(()).ok_or(E_PERM)
    }

    pub fn stop(&mut self) {
        self.streamer = None;
    }

    /// Whether the stream's peer is gone (`alive` asks the kernel), so the
    /// stream must be stopped; it is let go here.
    pub fn reap(&mut self, alive: impl Fn(u32) -> bool) -> bool {
        let gone = self.streamer.is_some_and(|p| !alive(p));
        if gone {
            self.streamer = None;
        }
        gone
    }
}
