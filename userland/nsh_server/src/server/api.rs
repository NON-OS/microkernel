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

//! What the host tells the server while the session runs: the shell's
//! output, each command line, each call the kernel refused, and the
//! answer to an escalation. Each is written to the record as it happens.

use super::Server;
use crate::record::EntryKind;
use crate::{Attestation, CallError, HostSigner, Rand, Roles};
use nsh_wire::record::MAX_COMMAND;
use nsh_wire::{Expect, Kind, MAX_DATA};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// The shell's output, sent in frames of at most 16384 bytes.
    pub fn output(&mut self, bytes: &[u8], now_ms: u64, rand: Rand<'_>) -> Result<(), CallError> {
        self.running()?;
        for chunk in bytes.chunks(MAX_DATA) {
            if let Err(why) = self.send(Kind::Output, chunk, Expect::ClientSession) {
                self.refuse(why, now_ms, rand);
                return Err(CallError::NotNow);
            }
            if let Some(l) = self.live.as_mut() {
                l.record.output(chunk);
            }
        }
        Ok(())
    }

    /// A command line the shell ran. A role that keeps digests, and a line
    /// longer than a record entry holds, keep its SHA-256 only.
    pub fn command(&mut self, line: &[u8], now_ms: u64, rand: Rand<'_>) -> Result<(), CallError> {
        let digest = self.running()?;
        if line.is_empty() {
            return Ok(());
        }
        // A record at its cap has already asked the host to stop the shell.
        let _ = match digest || line.len() > MAX_COMMAND {
            true => {
                self.entry(EntryKind::CommandDigest, now_ms, &nonos_hash::sha256(line), false, rand)
            }
            false => self.entry(EntryKind::Command, now_ms, line, false, rand),
        };
        Ok(())
    }
}
