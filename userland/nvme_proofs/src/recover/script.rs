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

//! A scripted controller for the recovery plan: what each step returns,
//! and the steps taken, in order.

use super::{recover, Recovered, Stuck, ABORT_GRACE_MS};
use crate::error::{NvmeError, NvmeResult};

pub(super) const TIMEOUT: NvmeResult<()> = Err(NvmeError::ControllerTimeout);
pub(super) const REFUSED: NvmeResult<()> = Err(NvmeError::AdminCommandFailed);

struct Script {
    csts: u32,
    abort: NvmeResult<bool>,
    steps: [NvmeResult<()>; 2],
    did: Vec<&'static str>,
}

impl Stuck for Script {
    fn csts(&mut self) -> u32 {
        self.did.push("csts");
        self.csts
    }
    fn abort(&mut self) -> NvmeResult<bool> {
        self.did.push("abort");
        self.abort
    }
    fn grace(&mut self, ms: u64) -> NvmeResult<()> {
        assert!(ms == 0 || ms == ABORT_GRACE_MS, "a grace of {ms} ms");
        self.did.push(if ms == 0 { "look" } else { "grace" });
        self.steps[0]
    }
    fn reset(&mut self) -> NvmeResult<()> {
        self.did.push("reset");
        self.steps[1]
    }
}

/// Recover from a controller reading `csts` whose Abort answers `abort`
/// (whether it aborted the command) and whose grace (or one look at the
/// ring) and reset return `steps`; how it ended, and the steps taken.
pub(super) fn run(
    csts: u32,
    abort: NvmeResult<bool>,
    steps: [NvmeResult<()>; 2],
) -> (Recovered, Vec<&'static str>) {
    let mut s = Script { csts, abort, steps, did: Vec::new() };
    (recover(&mut s), s.did)
}
