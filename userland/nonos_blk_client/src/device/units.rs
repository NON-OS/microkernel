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

//! Every disk behind one driver service: each unit asked in turn, a unit
//! with no disk left out and one that did not answer kept, so it is seen.
//! A driver that did not answer the transport at all is asked no further,
//! since every unit after it would wait out the same timeout. Pure, so the
//! host proofs mount it.

use alloc::vec::Vec;

use crate::error::BlkError;

pub fn each_unit<T>(
    units: u16,
    mut open: impl FnMut(u16) -> Result<Option<T>, BlkError>,
) -> Vec<(u16, Result<T, BlkError>)> {
    let mut out = Vec::new();
    for unit in 0..units {
        match open(unit) {
            Ok(Some(disk)) => out.push((unit, Ok(disk))),
            Ok(None) => {}
            Err(e @ BlkError::Transport(_)) => {
                out.push((unit, Err(e)));
                break;
            }
            Err(e) => out.push((unit, Err(e))),
        }
    }
    out
}
