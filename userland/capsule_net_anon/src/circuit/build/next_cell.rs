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

//! Taking the next cell that belongs to the circuit being built.

use crate::cell::{Cell, Frame};
use crate::link::Link;

use super::error::BuildError;
use super::timing::HOP_MS;

/*
 * One link carries every circuit, so waiting here also receives cells belonging
 * to a circuit that is already carrying traffic. They are handed back to the link
 * rather than dropped: each hop authenticates cells against a running digest over
 * everything it has received, so one cell skipped puts that hop permanently out of
 * step and every later cell on it fails to verify. Building a circuit must not
 * cost the circuit already running.
 */

pub(super) fn next(link: &mut Link, circuit: u32) -> Result<Cell, BuildError> {
    loop {
        let frame = link.recv(HOP_MS).map_err(|_| BuildError::Link)?;
        match frame {
            Some(Frame::Fixed(cell)) if cell.circuit == circuit => return Ok(cell),
            Some(other) => link.hold(other),
            None => return Err(BuildError::Timeout),
        }
    }
}
