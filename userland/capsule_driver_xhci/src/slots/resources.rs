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
use super::interrupt::InterruptEndpoint;
use crate::contexts::{HubInfo, Topology};
use crate::dma::DmaRegion;
use crate::rings::transfer::TransferRing;
use alloc::vec::Vec;

pub struct SlotResources {
    pub slot_id: u8,
    pub port_id: u8,
    pub speed: u8,
    /// The root port speaks USB 3 (Supported Protocol capability).
    pub usb3: bool,
    /// Where the device sits below its root port; 0 route on the port.
    pub topo: Topology,
    /// Set once Configure Hub has told the controller the device is a hub.
    pub hub: Option<HubInfo>,
    pub max_packet: u16,
    pub output_context: DmaRegion,
    pub input_context: DmaRegion,
    pub ep0: TransferRing,
    /// Each interrupt-IN endpoint configured, keyed by DCI. One slot used to
    /// hold a single one, and configuring a second interface replaced it:
    /// the first endpoint's ring was freed under a controller still
    /// scheduling it, and its device went silent.
    pub interrupt: Vec<InterruptEndpoint>,
    pub bulk: Option<crate::controller::BulkPipes>,
    /// A second bulk pair (OP_CONFIGURE_BULK2): UAS's other two pipes.
    pub bulk2: Option<crate::controller::BulkPipes>,
    pub isoch: Option<crate::controller::IsochPipe>,
    pub isoch_in: Option<crate::controller::IsochPipe>,
    /// What a failed controller may still reach, kept until Disable Slot
    /// drops the slot or a reset the table (`keep_if_failed`).
    pub quarantine: Vec<DmaRegion>,
}
impl SlotResources {
    /// The interrupt-IN endpoint at `dci`.
    pub fn interrupt_mut(&mut self, dci: u8) -> Option<&mut InterruptEndpoint> {
        self.interrupt.iter_mut().find(|ep| ep.dci == dci)
    }
}
