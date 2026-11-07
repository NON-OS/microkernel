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

//! The record sent as RECORD frames: parts of at most MAX_DATA - 1 bytes,
//! the last flagged.

use super::Server;
use crate::{Attestation, HostSigner, Roles, Why};
use alloc::vec::Vec;
use nsh_wire::{Expect, Kind, MAX_DATA};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// The record in parts of at most `MAX_DATA - 1` bytes, the last one
    /// flagged.
    pub(super) fn send_record(&mut self) -> Result<(), Why> {
        let Some(live) = self.live.as_ref() else { return Err(Why::Denied) };
        let parts: Vec<Vec<u8>> =
            live.record.bytes().chunks(MAX_DATA - 1).map(<[u8]>::to_vec).collect();
        let last = parts.len().saturating_sub(1);
        for (i, part) in parts.iter().enumerate() {
            let mut body = Vec::with_capacity(1 + part.len());
            body.push(u8::from(i == last));
            body.extend_from_slice(part);
            self.send(Kind::Record, &body, Expect::ClientSession)?;
        }
        Ok(())
    }
}
