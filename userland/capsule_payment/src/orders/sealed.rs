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

//! A variable-length sealed file (plan/29): its sealed length u64 LE, then
//! the record sealed under its own key. Absent is None; a file that does not
//! open is None too, for an offer or an escrow file nobody's money rests on
//! until its order is kept, and EIO for anything a caller must not lose.

use alloc::vec::Vec;

use nonos_payment_proto::EAGAIN;

use crate::disk::{self, Loaded};
use crate::keys::{open_record, seal_record};
use crate::wipe::wipe;

pub fn load_sealed(path: &[u8], record: &[u8]) -> Result<Option<Vec<u8>>, i32> {
    let head = match disk::load(path, 8) {
        Loaded::Silent => return Err(EAGAIN),
        Loaded::Absent => return Ok(None),
        Loaded::Bytes(b) => b,
    };
    let n = u64::from_le_bytes(head[..8].try_into().map_err(|_| EAGAIN)?) as usize;
    match disk::load(path, 8usize.saturating_add(n)) {
        Loaded::Bytes(b) => Ok(open_record(record, &b[8..]).ok()),
        Loaded::Absent => Ok(None),
        Loaded::Silent => Err(EAGAIN),
    }
}

/// Seal `plain` under `record` and save it; the caller's plain copy is wiped.
pub fn save_sealed(path: &[u8], record: &[u8], mut plain: Vec<u8>) -> Result<(), i32> {
    let sealed = seal_record(record, &plain);
    wipe(&mut plain);
    let sealed = sealed?;
    disk::save(path, &[&(sealed.len() as u64).to_le_bytes()[..], &sealed].concat())
}
