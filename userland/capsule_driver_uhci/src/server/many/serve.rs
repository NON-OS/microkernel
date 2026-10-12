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
//! What the front needs of one controller.
//! One request to the host: the port list from every controller, slots
//! given out here, an address request to the controller whose port it
//! names or a claim on a virtual port (address.rs), a park from
//! driver.usb_hub0 (park.rs), and anything else to the controller its
//! slot, or the parked slot a claimant's stands for, is bound to.

use super::address::address;
use super::front::Front;
use super::give_back::disable;
use super::hc::Hc;
use super::park::park;
use super::ports::ports;
use super::slots::{enable, forward};
use crate::server::reply::status;
use crate::wire::ops::*;
use crate::wire::Request;
use nonos_usb_virt::OP_PARK;

pub fn serve<H: Hc>(f: &mut Front<H>, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    match req.op {
        OP_HEALTHCHECK => status(tx, req, 0),
        OP_PORT_STATUS => ports(f, req, tx),
        OP_ENABLE_SLOT => enable(f, req, tx),
        OP_DISABLE_SLOT => disable(f, req, body, tx),
        OP_ADDRESS_DEVICE | OP_ADDRESS_ROUTED => address(f, req, body, tx),
        OP_PARK => park(f, req, body, tx),
        _ => forward(f, req, body, tx),
    }
}
