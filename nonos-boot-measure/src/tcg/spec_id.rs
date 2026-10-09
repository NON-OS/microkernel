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

use super::banks::{parse_banks, Banks};
use super::consts::{EV_NO_ACTION, MAX_EVENT_BYTES};
use super::error::LogError;
use super::grow::Walk;
use super::read::{whole, Cursor, Stop};

/*
 * The first event is a TCG_PCR_EVENT in the old SHA-1 shape, type EV_NO_ACTION,
 * whose body is the Spec ID structure. Walked over a prefix, it says how many
 * bytes the header needs when the prefix is too short to say what it is.
 */
pub fn spec_id_walk(log: &[u8]) -> Result<Walk<(Banks, usize)>, LogError> {
    let mut c = Cursor::new(log);
    let r = (|| {
        let (_pcr, kind) = (c.u32()?, c.u32()?);
        c.take(20)?;
        let size = c.u32()? as usize;
        if kind != EV_NO_ACTION {
            return Err(Stop::Bad(LogError::NotCryptoAgile));
        }
        if size > MAX_EVENT_BYTES {
            return Err(Stop::Bad(LogError::EventTooLarge));
        }
        let body = c.take(size)?;
        Ok(whole(parse_banks(body))?)
    })();
    Walk::of(r.map(|b| (b, c.at())))
}

/// The header of a log held whole: its banks and its length in bytes.
pub fn spec_id(log: &[u8]) -> Result<(Banks, usize), LogError> {
    spec_id_walk(log)?.done().ok_or(LogError::Truncated)
}
