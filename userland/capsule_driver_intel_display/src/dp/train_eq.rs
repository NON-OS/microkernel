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

use super::caps::Caps;
use super::lane::Drive;
use super::train::{Pattern, Source, TrainError};
use super::train_cr::{set, status};

const EQ_TRIES: u32 = 5;

pub fn equalise(
    s: &mut dyn Source,
    caps: &Caps,
    lanes: u32,
    mut d: Drive,
) -> Result<Drive, TrainError> {
    let (pattern, code) = if caps.tps3 { (Pattern::Tps3, 0x23) } else { (Pattern::Tps2, 0x22) };
    s.pattern(pattern);
    set(s, code, d, lanes)?;
    for _ in 0..EQ_TRIES {
        s.wait_us(caps.eq_interval_us);
        let st = status(s)?;
        if !st.cr_done(lanes) {
            return Err(TrainError::Failed);
        }
        if st.eq_done(lanes) {
            return Ok(d);
        }
        d = st.requested(lanes);
        s.drive(d);
        set(s, code, d, lanes)?;
    }
    Err(TrainError::Failed)
}
