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

use nonos_stark::field::P;

use super::layout::*;
use crate::words::{b32_at, u16_at, u32_at, u64_at, words_of_le};

/// A record read back. `None` for any length but 240, another magic or
/// version, an unknown flag, a reserved byte set, or an Hc word not below p:
/// one record has one encoding, so two byte strings never name one seal.
pub fn decode_record(b: &[u8]) -> Option<Record> {
    if b.len() != RECORD_BYTES || b.get(..4)? != MAGIC || u16_at(b, 4)? != VERSION {
        return None;
    }
    let flags = u16_at(b, FLAGS)?;
    if flags & !(FLAG_FINAL | FLAG_SATURATED | FLAG_OVERFLOW) != 0 {
        return None;
    }
    if RESERVED.iter().any(|&(lo, hi)| b[lo..hi].iter().any(|&x| x != 0)) {
        return None;
    }
    let hc = words_of_le(&b32_at(b, HC)?);
    if hc.iter().any(|&w| w >= P) {
        return None;
    }
    Some(Record {
        flags,
        sup_meas: b32_at(b, SUP_MEAS)?,
        sup_caps: u64_at(b, SUP_CAPS)?,
        authority: b[AUTHORITY],
        seal_no: u64_at(b, SEAL_NO)?,
        trace: b32_at(b, TRACE)?,
        rows: u64_at(b, ROWS)?,
        hc,
        meas: b32_at(b, MEAS)?,
        n_meas: u32_at(b, N_MEAS)?,
        app_id: b32_at(b, APP_ID)?,
    })
}
