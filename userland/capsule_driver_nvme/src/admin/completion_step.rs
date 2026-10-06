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

use crate::admin::Completion;
use crate::error::{NvmeError, NvmeResult};

/// What the entry at the completion queue head means for the one command
/// the driver has in flight.
#[derive(Clone, Copy, Debug)]
pub enum CqStep {
    /// The phase tag is last pass's: the controller has not written here.
    Empty,
    /// A new entry, but not for this command: another command id, or a
    /// submission queue this completion queue does not serve.
    Foreign,
    /// The entry completes this command: Ok only when no status bit is set.
    Done(NvmeResult<()>),
}

/// Read one entry against the expected phase, submission queue and command
/// id. A command id is only unique within its submission queue, so an entry
/// naming another queue is not this command's whatever its id. The status
/// field is everything above the phase bit (SC, SCT, CRD, More, DNR), and
/// any bit set there fails the command.
pub fn classify(c: Completion, phase: bool, sq_id: u16, cid: u16) -> CqStep {
    if c.phase() != phase {
        return CqStep::Empty;
    }
    if c.sq_id != sq_id || c.cid != cid {
        return CqStep::Foreign;
    }
    CqStep::Done(if c.successful() { Ok(()) } else { Err(NvmeError::AdminCommandFailed) })
}
