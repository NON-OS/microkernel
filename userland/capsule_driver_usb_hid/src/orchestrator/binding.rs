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

use alloc::vec::Vec;

use crate::descriptors::{HidBinding, HidKind};
use crate::xhci::{alloc_transfer_ring, control_transfer};

use super::enumerate::HidEndpoint;

const SET_IDLE: u8 = 0x0A;
const SET_PROTOCOL: u8 = 0x0B;

pub fn configure_binding(
    xhci_port: u32,
    slot: u8,
    root_port: u8,
    binding: HidBinding,
    out: &mut Vec<HidEndpoint>,
) {
    let iface = binding.interface_number as u16;
    let mut dummy = [0u8; 0];
    if matches!(binding.kind, HidKind::Keyboard | HidKind::Mouse) {
        // SET_PROTOCOL(boot): the reports are then the fixed boot layouts the
        // decoders read. A device that refuses it is still bound; the
        // controller driver recovers the stalled pipe.
        let _ = control_transfer(xhci_port, slot, 0x21, SET_PROTOCOL, 0, iface, 0, &mut dummy);
    }
    if binding.kind == HidKind::Keyboard {
        // SET_IDLE(0): report on change only, as Linux asks of keyboards.
        // Held keys repeat from here, so the default 500 ms resend is noise;
        // many devices STALL the request, which is fine.
        let _ = control_transfer(xhci_port, slot, 0x21, SET_IDLE, 0, iface, 0, &mut dummy);
    }
    let Ok(dci) = alloc_transfer_ring(
        xhci_port,
        slot,
        binding.endpoint_address,
        binding.max_packet_size,
        binding.interval,
    ) else {
        return;
    };
    out.push(HidEndpoint {
        port: xhci_port,
        root_port,
        slot,
        dci,
        kind: binding.kind,
        max_packet: binding.max_packet_size,
    });
}
