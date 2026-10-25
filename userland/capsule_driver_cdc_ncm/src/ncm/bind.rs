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

//! Binding an NCM device: the first configuration that has an NCM
//! function, whichever index it sits at (a phone may offer it after other
//! configurations), brought up by start.rs. A device
//! with only ECM (subclass 0x06) is the ECM driver's and is not taken.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::{Bind, Bus, Found, Station};

use super::function::find_ncm;
use super::link::Ncm;
use super::shape::tx_shape;
use super::start::start;

pub fn bind<B: Bus>(mut bus: B, found: &Found, drawn: Station) -> Bind<Ncm<B>> {
    let pick = |raw: &alloc::vec::Vec<u8>| Some((config_header(raw)?.1, find_ncm(raw)?));
    let Some((value, f)) = found.configs.iter().find_map(pick) else { return Bind::NotOurs };
    match start(&mut bus, value, &f, drawn) {
        Ok((taken, params, rx_max)) => {
            let Some(shape) = tx_shape(&params, f.pipes.max_packet_out) else {
                return Bind::Failed("NTB out size under one frame", E_IO);
            };
            // Without its notifications the link reads up, as it did before.
            let notify = f.notify.and_then(|ep| bus.configure_interrupt(ep).ok());
            let mut ncm = Ncm::new(bus, (drawn.bytes(), taken), f.pipes, rx_max, shape);
            (ncm.notify, ncm.ntb32) = (notify, params.ntb32());
            Bind::Ours(ncm)
        }
        Err((what, e)) => Bind::Failed(what, e),
    }
}
