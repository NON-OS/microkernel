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

//! REPORT LUNS on the well-known LU: every unit's LUN, single-level, then
//! any further entries the test gives as they are.

use super::scsi::{put, Sense};
use super::sim::Model;

impl Model {
    pub(super) fn report_luns(&self, data: Option<(u64, usize)>) -> Result<(), Sense> {
        let cfg = self.cfg.borrow();
        let mut entries: Vec<[u8; 8]> =
            cfg.lus.iter().map(|l| [0, l.lun, 0, 0, 0, 0, 0, 0]).collect();
        entries.extend(cfg.extra_entries.iter().copied());
        let mut list = ((entries.len() * 8) as u32).to_be_bytes().to_vec();
        list.extend([0u8; 4]);
        entries.iter().for_each(|e| list.extend(e));
        put(data, &list);
        Ok(())
    }
}
