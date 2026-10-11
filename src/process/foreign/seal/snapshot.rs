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

//! A family's record as it stands: what finish seals with FINAL set, and
//! what the live read hands DELIVER with it clear.

use super::family::Family;
use super::record::{encode, Facts, OVERFLOW, RECORD_BYTES, SATURATED};

pub(super) fn record_of(
    fam: &Family,
    final_flag: u16,
    seal_no: u64,
    app_id: [u8; 32],
    hc: [u64; 4],
) -> [u8; RECORD_BYTES] {
    let flag = |on: bool, f: u16| if on { f } else { 0 };
    encode(&Facts {
        flags: final_flag | flag(fam.saturated, SATURATED) | flag(fam.overflow, OVERFLOW),
        sup_meas: fam.sup_meas,
        sup_caps: fam.sup_caps,
        authority: fam.authority,
        seal_no,
        trace: fam.trace,
        rows: fam.rows,
        hc,
        meas: fam.meas,
        n_meas: fam.n_meas,
        app_id,
    })
}
