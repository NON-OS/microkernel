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

//! The EDID read over AUX for the proofs that drive it on a sink of their
//! own: segment 0, and a deadline that never comes, so each such proof
//! holds the read's retries and lengths and not its clock.

use crate::dp::aux::{Aux, AuxError};
use crate::engine::access::Until;
use crate::model::Ticks;

pub fn read_edid(aux: &mut dyn Aux, offset: u8, out: &mut [u8]) -> Result<(), AuxError> {
    let t = Ticks::new();
    crate::dp::i2c_edid::read_edid(aux, 0, offset, out, &Until::new(&t, u64::MAX))
}
