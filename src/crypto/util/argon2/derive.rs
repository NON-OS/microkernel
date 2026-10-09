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

//! Argon2 (RFC 9106, version 0x13) over memory the heap lends and takes
//! back wiped. At most `MAX_M_KIB` of it is ever asked for.

use super::ends::{first_block, h0, tag, Inputs};
use super::fill::fill_all;
use super::index::{Shape, SLICES};
use super::memory::Memory;
use super::params::{Argon2Error, Params};
use super::wipe::wipe;
use crate::crypto::constant_time::secure_zero;

/// Derive `out.len()` bytes (at least 4) from `inputs` under `params`.
/// `between` is called after every segment, so a long derivation can let
/// the caller answer what cannot wait.
pub fn argon2(
    inputs: &Inputs,
    params: Params,
    out: &mut [u8],
    between: &mut dyn FnMut(),
) -> Result<(), Argon2Error> {
    let blocks = params.blocks()?;
    if out.len() < 4 || out.len() > u32::MAX as usize {
        return Err(Argon2Error::BadParams);
    }
    let lanes = params.p as usize;
    let lane_len = blocks / lanes;
    let shape = Shape {
        lanes,
        lane_len,
        seg_len: lane_len / SLICES,
        blocks,
        passes: params.t,
        y: inputs.y,
    };
    let mut mem = Memory::new(blocks)?;
    let mut seed = h0(inputs, params, out.len());
    for lane in 0..lanes {
        for column in 0..2 {
            first_block(&seed, column as u32, lane as u32, mem.at_mut(lane * lane_len + column));
        }
    }
    secure_zero(&mut seed);
    fill_all(&shape, &mut mem, between);
    let mut last = *mem.at(lane_len - 1);
    for lane in 1..lanes {
        let other = mem.at(lane * lane_len + lane_len - 1);
        last.iter_mut().zip(other.iter()).for_each(|(a, b)| *a ^= b);
    }
    tag(&last, out);
    wipe(&mut last);
    Ok(())
}
