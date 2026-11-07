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

//! What the host tells the server before a connection: its enrolment
//! certificate and the bounds it holds every connection to.

use alloc::vec::Vec;
use nsh_wire::record::MAX_RECORD;

/// The smallest record cap a host may set: room for a header at its
/// largest, the entries every session ends with, and the footer.
pub const MIN_RECORD: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// The host's `nsh-enrol/1` certificate, signatures and all.
    pub enrol: Vec<u8>,
    /// How long the handshake may take, from the first byte to `AUTH_OK`.
    pub handshake_ms: u64,
    /// The idle timeout `AUTH_OK` states, in seconds (1 to 86400).
    pub idle_s: u32,
    /// A checkpoint is written after every this many record entries; 0
    /// writes none.
    pub ckpt_every: u32,
    /// The most bytes one session's record may hold, `MIN_RECORD` to
    /// 64 MiB. The server keeps the record in memory until it is sent.
    pub max_record: usize,
}

impl Config {
    /// Whether every bound is one the server can keep.
    pub fn valid(&self) -> bool {
        (1..=nsh_wire::MAX_IDLE_S).contains(&self.idle_s)
            && self.handshake_ms > 0
            && (MIN_RECORD..=MAX_RECORD).contains(&self.max_record)
            && !self.enrol.is_empty()
            && nsh_wire::Enrol::parse(&self.enrol).is_ok()
    }
}

/// The host's configuration is one the server cannot keep: a bound out of
/// range, or an enrolment certificate that does not parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadConfig;
