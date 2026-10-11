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

//! A rig whose slot is a removable one with an SD card in it.

use super::host::HostCfg;
use super::sd::{SdCard, SdCfg};
use super::{CardCfg, Rig, SimBus, TestClock, TestLog};
use crate::emmc::slot::HostParts;

/// A removable SD slot as QEMU's sdhci-pci reports it, ADMA2 and 64-bit
/// added: spec 3.00, 200 MHz base clock, high speed, 3.3 V only, slot
/// type 0 (removable).
pub const SD_SLOT_CAPS: u32 = (200 << 8) | 1 << 19 | 1 << 21 | 1 << 24 | 1 << 28;

pub fn sd_slot() -> HostCfg {
    HostCfg { caps: SD_SLOT_CAPS, ..HostCfg::default() }
}

impl Rig {
    /// The SD card answers every command; the eMMC model the rig also
    /// holds is never reached.
    pub fn sd(host: HostCfg, card: SdCfg) -> Self {
        let rig = Rig::new(host, CardCfg::default());
        rig.sim.borrow_mut().sd = Some(SdCard::new(card));
        rig
    }

    /// A removable slot with nothing in it.
    pub fn empty_slot(host: HostCfg) -> Self {
        let card = CardCfg { absent: true, ..CardCfg::default() };
        Rig::new(HostCfg { cd_inserted: false, ..host }, card)
    }

    /// What a slot makes its hosts from.
    pub fn parts(&self, intel_emmc: bool) -> HostParts<SimBus, TestClock, TestLog> {
        let (io, clock, log) =
            (SimBus(self.sim.clone()), TestClock(self.now.clone()), self.log.clone());
        HostParts { io, clock, log, table: self.table, intel_emmc }
    }
}
