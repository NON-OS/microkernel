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

//! What carries a conversation's stream, as the conversation sees it.

extern crate alloc;

use alloc::vec::Vec;

/// How far the exit has got with a stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Far {
    Opening,
    Open,
    Ended(u8),
    Gone,
}

/// Why bytes did not go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unsent {
    /// No window left; try again once the exit grants more.
    Blocked,
    /// The stream is over.
    Over,
}

/// How far the network that carries the streams has got, for a caller to
/// show while it waits: whether a stream can be opened now, and the step
/// reached of how many. net.anon counts five: the authorities' keys, the
/// relay list, the relays' descriptors, a link to a guard, a first circuit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    pub ready: bool,
    pub step: u8,
    pub steps: u8,
}

/// What carries a conversation's stream.
pub trait Tunnel {
    /// Open a stream to `host` on `port`, or the SOCKS reply code that
    /// says why not.
    fn open(&mut self, host: &[u8], port: u16) -> Result<u16, u8>;
    fn far(&self, id: u16) -> Far;
    fn send(&mut self, id: u16, data: &[u8]) -> Result<usize, Unsent>;
    /// Take up to `max` bytes that have arrived on the stream.
    fn take(&mut self, id: u16, max: usize) -> Vec<u8>;
    /// Bytes that have arrived on the stream and not been taken.
    fn waiting(&self, id: u16) -> usize;
    /// End the stream if it is still open and forget it.
    fn close(&mut self, id: u16);
    /// How far the network has got.
    fn progress(&self) -> Progress;
}

impl Progress {
    /// The steps net.anon counts.
    pub const STEPS: u8 = 5;

    /// How far the network has got, from what the manager holds: the
    /// directory stage (0 cold, 1 anchored, 2 joining, 3 ready), whether a
    /// consensus already held is being refreshed (it serves meanwhile),
    /// whether a stream could be opened now as far as the directory and the
    /// link go, whether a link to a guard is up, and how many circuits are
    /// open. Ready only with an open circuit: a CONNECT before one is
    /// refused as not connected yet.
    pub fn of(stage: u8, refreshing: bool, opens: bool, linked: bool, circuits: usize) -> Progress {
        let step = match (stage, refreshing) {
            (0, false) => 1,
            (1, false) => 2,
            (2, false) => 3,
            _ if !linked => 4,
            _ => 5,
        };
        Progress { ready: opens && circuits > 0, step, steps: Progress::STEPS }
    }
}
