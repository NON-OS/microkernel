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

//! When a command given up on stops being the controller's. Only its
//! completion says so, and any completion does: once the controller has
//! written the entry it is done with the data buffer, whatever the status.
//! That status belongs to the request already answered when the wait ran
//! out, so it fails nothing now. A late error used to keep the command out
//! after its entry was consumed, and every later request then waited 30 s
//! for an entry that had already come. Pure, so the host proofs hold it.

use crate::error::{NvmeError, NvmeResult};

/// Settle the command `out` names, if any, by `wait`ing for its completion.
/// A completion of any status retires it. Anything else, a wait that ran out
/// above all, leaves it out: the controller may still be moving its data.
pub fn settled<W>(out: &mut Option<u16>, wait: W) -> NvmeResult<()>
where
    W: FnOnce(u16) -> NvmeResult<()>,
{
    let Some(cid) = *out else { return Ok(()) };
    match wait(cid) {
        Ok(()) | Err(NvmeError::AdminCommandFailed) => {
            *out = None;
            Ok(())
        }
        Err(e) => Err(e),
    }
}
