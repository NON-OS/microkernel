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

//! What a reply is read from, and what reading it brings.

use alloc::vec::Vec;

/// A stream a reply is read from: `RouteStream` in the capsule, a script in
/// the proofs.
pub trait Source {
    /// What has arrived, waiting up to `wait_ms` for the first of it: zero
    /// when nothing came in that time, or once the far end has finished.
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str>;
    /// The far end finished and all it sent has been read.
    fn ended(&self) -> bool;
}

/// What a read brought.
#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    /// The bytes that came, whole or not; the HTTP framing judges them.
    Got(Vec<u8>),
    /// Nothing came, or the stream broke before anything did.
    Nothing,
    /// More came than `max`: no index or package is that large.
    TooLarge,
}
