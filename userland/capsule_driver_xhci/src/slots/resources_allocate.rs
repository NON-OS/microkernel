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
use super::resources::SlotResources;
use crate::contexts::{
    device_context_bytes, input_context_bytes, max_packet_for_speed, write_address_device_input,
    Place,
};
use crate::dma::DmaPool;
use crate::error::XhciResult;
use crate::rings::transfer::TransferRing;
use alloc::vec::Vec;

impl SlotResources {
    /// The contexts and EP0 ring of `slot_id`, with the input context
    /// Address Device reads for a device at `place`.
    pub fn allocate(
        pool: &DmaPool,
        context_size: u8,
        slot_id: u8,
        place: Place,
        usb3: bool,
    ) -> XhciResult<Self> {
        let output_context = pool.alloc(device_context_bytes(context_size))?;
        let input_context = pool.alloc(input_context_bytes(context_size))?;
        let ep0 = TransferRing::new(pool)?;
        output_context.zero();
        let max_packet = max_packet_for_speed(place.speed, usb3);
        write_address_device_input(&input_context, context_size, &place, max_packet, ep0.phys());
        Ok(Self {
            slot_id,
            port_id: place.root_port,
            speed: place.speed,
            usb3,
            topo: place.topo,
            hub: None,
            max_packet,
            output_context,
            input_context,
            ep0,
            interrupt: Vec::new(),
            bulk: None,
            quarantine: Vec::new(),
        })
    }
}
