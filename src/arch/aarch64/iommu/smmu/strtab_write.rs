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

use crate::memory::iommu::IommuError;

use super::cmdq::Cmdq;
use super::encode::{cmd, ste};
use super::mmio::Regs;
use super::strtab::{StreamTable, SPLIT};
use super::{frame, wait};

impl StreamTable {
    /// The level 2 table holding `sid`, made and hooked in if missing.
    /// Every STE in a new table aborts until one is written.
    fn level2(&mut self, sid: u32, q: &mut Cmdq, regs: Regs) -> Result<u64, IommuError> {
        let i = (sid >> SPLIT) as usize;
        if self.l2[i] == 0 {
            let table = frame::alloc()?;
            for entry in frame::words(table)?.chunks_exact_mut(8) {
                entry.copy_from_slice(&ste::abort());
            }
            wait::publish();
            frame::words(self.l1)?[i] = ste::l1_descriptor(table, SPLIT);
            self.l2[i] = table;
            q.run(regs, &[cmd::cfgi_all()])?;
        }
        Ok(self.l2[i])
    }

    pub(super) fn write(
        &mut self,
        sid: u32,
        ste: ste::Ste,
        q: &mut Cmdq,
        regs: Regs,
    ) -> Result<(), IommuError> {
        if !self.covers(sid) {
            return Err(IommuError::InvalidDevice);
        }
        let table = self.level2(sid, q, regs)?;
        let at = ((sid & ((1 << SPLIT) - 1)) * 8) as usize;
        super::ste_write::write(&mut frame::words(table)?[at..at + 8], &ste);
        // The CD goes too: a stream moved to another domain must not keep
        // walking with the CD the unit cached from its last one.
        q.run(regs, &[cmd::cfgi_ste(sid), cmd::cfgi_cd(sid)])
    }
}
