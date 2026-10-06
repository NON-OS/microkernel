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

//! A directory fetch in flight, and the directory cursors beside it.

extern crate alloc;

use alloc::vec::Vec;

/// How long a connection may take to open.
pub(super) const ESTABLISH_MS: i64 = 8_000;

/// How long the request may take to be taken by the socket.
pub(super) const SEND_MS: i64 = 10_000;

/// How long the body may take. A consensus is 370 kB on the wire and
/// authorities are often loaded.
pub(super) const READ_MS: i64 = 30_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Stage {
    Opening,
    Sending,
    Reading,
}

pub struct Job {
    pub(super) handle: u32,
    pub(super) address: [u8; 4],
    pub(super) dir_port: u16,
    pub(super) request: Vec<u8>,
    pub(super) sent: usize,
    pub(super) stage: Stage,
    /// When the current stage gives up, in uptime milliseconds.
    pub(super) deadline: i64,
    /// Whether the connection has been seen on its way up, so that a later
    /// Closed means refused rather than not polled yet.
    pub(super) opening: bool,
    pub(super) raw: Vec<u8>,
}

/// What the directory bootstrap holds between turns.
#[derive(Default)]
pub struct DirWork {
    /// The fetch in flight, if any.
    pub job: Option<Job>,
    /// Which authority a certificate or consensus sweep asks next.
    pub sweep: usize,
    /// Which microdescriptor batch is asked next.
    pub micro: usize,
}
