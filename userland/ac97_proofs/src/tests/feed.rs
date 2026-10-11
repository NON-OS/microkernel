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

//! The audio server's side of a playing stream, as a test drives it.

use super::util::{ask, request, status};
use crate::constants::ring::PERIOD_BYTES;
use crate::model::Shared;
use crate::protocol::MAX_PCM_CHUNK;
use crate::server::{refill, Stream};
use crate::setup::Driver;

/// Byte `i` of a stream no period boundary can line up by accident.
pub fn pattern(i: usize) -> u8 {
    (i.wrapping_mul(7) ^ (i >> 13)) as u8
}

/// Feed the stream as the audio server does, 4 KiB at a time until the
/// queue says again, then let the engine play a period and refill.
pub fn period(d: &mut Driver, st: &mut Stream, ich: &Shared, fed: &mut usize, total: usize) {
    while *fed < total {
        let n = MAX_PCM_CHUNK.min(total - *fed);
        let chunk: Vec<u8> = (*fed..*fed + n).map(pattern).collect();
        if status(&ask(d, st, &request(7, 1, &chunk))) != 0 {
            break;
        }
        *fed += n;
    }
    ich.borrow_mut().play(PERIOD_BYTES);
    refill(d, st);
}
