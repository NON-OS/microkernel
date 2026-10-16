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

//! Binding an RNDIS device (Remote NDIS 1.0, 3.1): the function found in
//! a configuration, brought up (start.rs), and served with its link as
//! the device last indicated it, up when it said nothing.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::{Bind, Bus, Found};

use super::function::find_rndis;
use super::link::Rndis;
use super::start::start;

pub fn bind<B: Bus>(mut bus: B, found: &Found) -> Bind<Rndis<B>> {
    let pick = |raw: &alloc::vec::Vec<u8>| Some((config_header(raw)?.1, find_rndis(raw)?));
    let Some((value, f)) = found.configs.iter().find_map(pick) else { return Bind::NotOurs };
    match start(&mut bus, value, &f) {
        Ok((mac, limits, media)) => {
            // Without its notifications the link reads up, as it did before.
            let notify = f.notify.and_then(|ep| bus.configure_interrupt(ep).ok());
            let mut nic = Rndis::new(bus, mac, f.pipes, limits);
            (nic.comm, nic.notify) = (f.comm, notify);
            nic.link = media.unwrap_or(true);
            Bind::Ours(nic)
        }
        Err((what, e)) => Bind::Failed(what, e),
    }
}
