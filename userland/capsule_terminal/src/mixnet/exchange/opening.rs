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

//! Stepping a tunnel open, one slice of SOCKS round trips at a time.

use alloc::vec::Vec;

use nonos_route_link::Slice;

use super::open::SLICE_MS;
use super::types::{Exchange, Phase};

impl Exchange {
    pub(super) fn opening(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let Phase::Opening(opening) = &mut self.phase else {
            return Ok(None);
        };
        match opening.step(SLICE_MS) {
            Slice::Waiting => Ok(None),
            Slice::Failed(why) => Err(why),
            Slice::Done => {
                let Phase::Opening(opening) = core::mem::replace(&mut self.phase, Phase::Start)
                else {
                    return Ok(None);
                };
                self.stream = Some(opening.into_stream());
                self.connected()
            }
        }
    }
}
