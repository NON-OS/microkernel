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

//! Binding a Bluetooth controller: its configuration chosen, its three
//! endpoints given to driver.xhci0, then the configuration set. The
//! controller learns of the pipes before the device is told to use them
//! (xHCI 1.2, 4.3.5), as the USB network drivers do it.

use super::choose::{choose, Verdict};
use super::ops::configure_interrupt;
use super::pipes::XhciPipes;
use crate::run::Desk;
use nonos_bt_hci::usb::BtInterface;
use nonos_usbnet::xhci::{XhciBus, E_INVAL};
use nonos_usbnet::{Bind, Bus, Found, Pipes, Setup};

pub struct Bound {
    pub pipes: XhciPipes,
    pub iface: BtInterface,
}

pub fn bind(mut bus: XhciBus, found: &Found, desk: Desk) -> Bind<Bound> {
    let c = match choose(&found.configs) {
        Verdict::Ours(c) => c,
        Verdict::NotOurs => return Bind::NotOurs,
        Verdict::Unusable(why) => return Bind::Failed(why, E_INVAL),
    };
    let dci = match configure_interrupt(bus.xhci, bus.slot, &c.iface.events) {
        Ok(dci) => dci,
        Err(e) => return Bind::Failed("event endpoint refused", e),
    };
    let (i, o) = (c.iface.acl_in, c.iface.acl_out);
    // bMaxBurst 0: the ACL endpoints of a USB 2.0 controller have no
    // SuperSpeed companion.
    let (burst_in, burst_out) = (0, 0);
    let (bulk_in, bulk_out, max_packet_in, max_packet_out) =
        (i.address, o.address, i.max_packet, o.max_packet);
    let p = Pipes { bulk_in, bulk_out, max_packet_in, max_packet_out, burst_in, burst_out };
    if let Err(e) = bus.configure_bulk(&p) {
        return Bind::Failed("ACL endpoints refused", e);
    }
    if let Err(e) = bus.control_out(Setup::set_configuration(c.value), &[]) {
        return Bind::Failed("SET_CONFIGURATION refused", e);
    }
    let pipes = XhciPipes { bus, dci, event_read: c.event_read, desk };
    Bind::Ours(Bound { pipes, iface: c.iface })
}
