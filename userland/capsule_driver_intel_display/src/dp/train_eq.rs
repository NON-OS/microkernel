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

//! Channel equalisation (DisplayPort 1.4a section 3.5.1.2.2): TPS3 when
//! the sink has it, else TPS2, unscrambled, the sink's read interval
//! between tries. A lane losing clock recovery fails the phase, as do
//! five tries without lock and alignment.

use super::aux::Aux;
use super::caps::Caps;
use super::lane::Drive;
use super::train::{Link, Pattern, TrainError};
use super::train_cr::{set, status};

const EQ_TRIES: u32 = 5;

pub fn equalise(
    aux: &mut dyn Aux,
    tx: &mut dyn Link,
    caps: &Caps,
    lanes: u32,
    mut d: Drive,
) -> Result<Drive, TrainError> {
    let (pattern, code) = if caps.tps3 { (Pattern::Tps3, 0x23) } else { (Pattern::Tps2, 0x22) };
    tx.pattern(pattern);
    set(aux, code, d, lanes)?;
    for _ in 0..EQ_TRIES {
        tx.wait_us(caps.eq_interval_us);
        let s = status(aux)?;
        if !s.cr_done(lanes) {
            return Err(TrainError::Failed);
        }
        if s.eq_done(lanes) {
            return Ok(d);
        }
        d = s.requested(lanes);
        tx.drive(d);
        set(aux, code, d, lanes)?;
    }
    Err(TrainError::Failed)
}
