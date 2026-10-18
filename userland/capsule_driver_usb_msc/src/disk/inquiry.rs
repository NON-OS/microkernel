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

//! INQUIRY, the first command a unit is sent. SPC makes it the one command
//! every logical unit answers whatever its state, and some USB storage
//! firmware does not answer anything else correctly until it has seen it
//! (as observed, unverified). What it says names the unit for as long
//! as it is served (scan/identity.rs).

use super::digest::digest;
use super::sense::sense;
use super::transport::command;
use super::types::{Data, Disk};
use crate::scsi::{inquiry as inquiry_cdb, INQUIRY_DATA_LEN};
use crate::state::State;

/// An INQUIRY, sent once more after the sense of a CHECK CONDITION is taken
/// (a power-on said first, say);
/// taking the sense clears the condition, which was left pending for the
/// commands after it. Returns the digest of what a good INQUIRY says the
/// unit is (its type, vendor, product and revision), or None when both
/// failed, which still counts as asked: a failed one's data names nothing.
pub fn inquiry(disk: &Disk, state: &mut State) -> Result<Option<u64>, i32> {
    for _ in 0..2 {
        let mut data = [0u8; INQUIRY_DATA_LEN];
        if command(disk, state, inquiry_cdb(), Data::In(&mut data))?.0 == 0 {
            return Ok(Some(digest(&data)));
        }
        sense(disk, state)?;
    }
    Ok(None)
}
