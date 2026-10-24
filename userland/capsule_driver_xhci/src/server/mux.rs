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

//! Several xHCI controllers behind the one `driver.xhci0` endpoint: a
//! chipset controller beside a Thunderbolt or USB4 one, or an AMD APU's
//! two. Root ports are numbered across the controllers (the primary's from
//! 1, the next one's after its last), and a class driver's slot id is the
//! multiplexer's own, reserved at Enable Slot and bound to a controller's
//! slot at Address Device, once the port names the controller. With one
//! controller every request goes to it unchanged.

use alloc::vec::Vec;

pub(super) use super::mux_forward::VSlot;
use super::mux_ports::port_ranges;
pub(super) use super::mux_status::status;
use crate::protocol::{
    Request, OP_ADDRESS_DEVICE, OP_ADDRESS_ROUTED, OP_CONTROLLER_STATUS, OP_DISABLE_SLOT,
    OP_ENABLE_SLOT, OP_HEALTHCHECK, OP_PORT_STATUS,
};
use crate::server::context::Context;
use crate::server::dispatch::dispatch;
use nonos_usb_virt::Virt;

pub struct Mux {
    pub ctxs: Vec<Context>,
    /// Each controller's first global port number less one, and its port count.
    pub(super) ports: Vec<(u8, u8)>,
    pub(super) vslots: [VSlot; 256],
    /// Each controller's `generation` when its bound slot ids were last
    /// checked against it (`forget_revived`).
    pub(super) generations: Vec<u32>,
    /// Children below hubs, parked by driver.usb_hub0 (mux_virt.rs).
    pub(super) virt: Virt,
    /// Shared devices' interfaces and pipes (mux_claims.rs, mux_police.rs).
    pub(super) share: crate::slots::Sharing,
}

impl Mux {
    pub fn new(ctxs: Vec<Context>) -> Self {
        let counts: Vec<u8> = ctxs.iter().map(|c| c.driver.layout.max_ports).collect();
        let generations = ctxs.iter().map(|c| c.driver.generation).collect();
        Self {
            ports: port_ranges(&counts),
            ctxs,
            vslots: [VSlot::Free; 256],
            generations,
            virt: Virt::default(),
            share: Default::default(),
        }
    }

    pub(super) fn inner(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        self.forget_revived();
        if self.ctxs.len() == 1 {
            return dispatch(&mut self.ctxs[0], req, body, tx);
        }
        match req.op {
            OP_HEALTHCHECK | OP_CONTROLLER_STATUS => dispatch(&mut self.ctxs[0], req, body, tx),
            OP_PORT_STATUS if body.is_empty() => self.port_status(req, tx),
            OP_ENABLE_SLOT if body.is_empty() => self.enable_slot(req, tx),
            OP_DISABLE_SLOT => self.disable_slot(req, body, tx),
            OP_ADDRESS_DEVICE | OP_ADDRESS_ROUTED => self.address(req, body, tx),
            _ => self.forward(req, body, tx),
        }
    }
}
