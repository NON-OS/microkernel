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

//! The descriptions the info ops reply with.

use super::super::env::{Clock, Log, Mmio};
use super::super::info::{
    controller_info, names, port_entry, Names, CONTROLLER_INFO_LEN, PORT_ENTRY_LEN,
};
use super::super::mmc::ops;
use super::EmmcDisk;

impl<M: Mmio, C: Clock, L: Log> EmmcDisk<M, C, L> {
    /// The model and serial IDENTIFY carries.
    pub fn names(&self) -> Names {
        names(&self.card.cid)
    }

    pub fn controller_info(&self) -> [u8; CONTROLLER_INFO_LEN] {
        controller_info(&self.host.caps, &self.host.snapshot())
    }

    /// The PORT_LIST entry, with the card's status read now.
    pub fn port_entry(&mut self) -> [u8; PORT_ENTRY_LEN] {
        let status = ops::status(&mut self.host, self.card.rca);
        let snap = self.host.snapshot();
        port_entry(status.is_ok(), &snap, self.card.ocr, status.unwrap_or(0))
    }
}
