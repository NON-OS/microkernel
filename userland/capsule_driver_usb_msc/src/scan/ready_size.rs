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

//! The check's one question to a bound device (`check`): is it ready, and
//! what size does it say it is now.

use crate::disk::{capacity, test_ready, Disk, Unit};
use crate::protocol::{E_IO, E_NOMEDIUM};
use crate::state::State;

/// UNIT ATTENTIONs a TEST UNIT READY is sent again past: a reader that
/// starts its card again after a reset says a power-on, then a medium
/// change (as observed, unverified), one UNIT ATTENTION per command.
const ATTENTIONS: usize = 2;

/// Whether `disk` is ready, and the size it says now. What a UNIT
/// ATTENTION said on the way is kept in `state` (`Said`) for the check.
pub(super) fn ready_size(disk: &Disk, state: &mut State) -> Result<(u64, u32), i32> {
    let mut said = test_ready(disk, state, &mut false)?;
    for _ in 0..ATTENTIONS {
        if said != Unit::Attention {
            break;
        }
        said = test_ready(disk, state, &mut false)?;
    }
    match said {
        Unit::Ready => capacity(disk, state),
        Unit::NoMedium => Err(E_NOMEDIUM),
        Unit::Attention | Unit::NotYet => Err(E_IO),
    }
}
