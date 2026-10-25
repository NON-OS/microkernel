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

use super::layout::*;
use crate::words::words_le;

impl Record {
    pub fn encode(&self) -> [u8; RECORD_BYTES] {
        let mut b = [0u8; RECORD_BYTES];
        b[..4].copy_from_slice(MAGIC);
        b[4..6].copy_from_slice(&VERSION.to_le_bytes());
        b[FLAGS..FLAGS + 2].copy_from_slice(&self.flags.to_le_bytes());
        b[SUP_MEAS..SUP_MEAS + 32].copy_from_slice(&self.sup_meas);
        b[SUP_CAPS..SUP_CAPS + 8].copy_from_slice(&self.sup_caps.to_le_bytes());
        b[AUTHORITY] = self.authority;
        b[SEAL_NO..SEAL_NO + 8].copy_from_slice(&self.seal_no.to_le_bytes());
        b[TRACE..TRACE + 32].copy_from_slice(&self.trace);
        b[ROWS..ROWS + 8].copy_from_slice(&self.rows.to_le_bytes());
        b[HC..HC + 32].copy_from_slice(&words_le(&self.hc));
        b[MEAS..MEAS + 32].copy_from_slice(&self.meas);
        b[N_MEAS..N_MEAS + 4].copy_from_slice(&self.n_meas.to_le_bytes());
        b[APP_ID..APP_ID + 32].copy_from_slice(&self.app_id);
        b
    }
}
