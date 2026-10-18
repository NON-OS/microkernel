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

//! One host's slot: a fixed one, whose card must come up, or a removable
//! one, served with a card in it or without.

use super::super::disk::{EmmcDisk, MAX_SECTORS, SECTOR_SIZE};
use super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::error::EmmcResult;
use super::parts::HostParts;

pub struct Slot<M: Mmio, C: Clock, L: Log> {
    pub disk: Option<EmmcDisk<M, C, L>>,
    pub removable: bool,
    pub(super) parts: HostParts<M, C, L>,
    pub(super) data: DmaBuf,
    /// A bring-up failed on the card now in the slot.
    pub(super) tried: bool,
    /// A card came or went since the last GEOMETRY.
    pub(super) changed: bool,
}

impl<M: Mmio + Clone, C: Clock + Clone, L: Log + Clone> Slot<M, C, L> {
    /// The board's eMMC, or a known Intel eMMC host: the card comes up now
    /// or the slot is not served, as before removable slots were.
    pub fn fixed(parts: HostParts<M, C, L>, data: DmaBuf) -> EmmcResult<Self> {
        let disk = EmmcDisk::bring_up(EmmcDisk::reset(parts.host())?, data)?;
        Ok(Self { disk: Some(disk), removable: false, parts, data, tried: false, changed: false })
    }

    /// A card reader's slot: the host is reset and watches its card detect,
    /// and a card in it now is brought up. Fails only if the host does.
    pub fn removable(parts: HostParts<M, C, L>, data: DmaBuf) -> EmmcResult<Self> {
        let mut host = parts.host();
        host.init()?;
        host.quiesce();
        let mut slot =
            Self { disk: None, removable: true, parts, data, tried: false, changed: true };
        slot.arm();
        slot.refresh();
        Ok(slot)
    }

    /// The most sectors one request moves, with a card in the slot or not.
    pub fn max_sectors(&self) -> u32 {
        core::cmp::min(MAX_SECTORS as usize, self.data.len / SECTOR_SIZE) as u32
    }

    /// Stop the host before its DMA regions go.
    pub fn quiesce(&mut self) {
        match self.disk.as_mut() {
            Some(d) => d.quiesce(),
            None => self.parts.host().quiesce(),
        }
    }
}
