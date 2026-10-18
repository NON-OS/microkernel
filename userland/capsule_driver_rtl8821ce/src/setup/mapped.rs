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

//! What a claimed and mapped chip carries into start-up.

use crate::regs::Regs;

pub struct Mapped {
    pub regs: Regs,
    pub device_id: u64,
    pub claim_epoch: u64,
    /// The board facts, read straight after power-on and before the firmware
    /// download and the MAC tables run. rtw88 reads the efuse at that point and
    /// only brings the MAC up afterwards; taking the read at the end instead found
    /// the efuse registers answering zero on real silicon, so it is taken here,
    /// while the chip is still in the state the reference driver reads it in.
    pub efuse: Option<crate::efuse::EfuseInfo>,
}
