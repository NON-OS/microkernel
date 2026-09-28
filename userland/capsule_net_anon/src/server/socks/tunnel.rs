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
}
