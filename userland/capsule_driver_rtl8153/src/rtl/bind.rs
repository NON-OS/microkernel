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

//! Binding an adapter: a listed id with a vendor configuration; the chip's
//! version read and judged; every interface held; the pipes given to the
//! host and the vendor configuration set; then the chip woken, given the
//! drawn station address, its PHY sent into autonegotiation, its MAC
//! started, and its link read once. Each step that stops the bind is named.

use nonos_usbnet::{Bind, Bus, Found, Station};

use super::adapters::listed;
use super::bring_up::bring_up;
use super::claim::Claim;
use super::config::vendor_config;
use super::pause::Pause;
use super::state::Rtl;
use super::watch::look;

pub fn bind<B: Bus + Claim + Pause>(mut bus: B, found: &Found, drawn: Station) -> Bind<Rtl<B>> {
    if !listed(found.info.vendor, found.info.product) {
        return Bind::NotOurs;
    }
    let Some(v) = vendor_config(found) else { return Bind::NotOurs };
    let mac = drawn.bytes();
    match bring_up(&mut bus, found.port, &v, mac) {
        Ok((model, page)) => {
            let mut rtl = Rtl::new(bus, mac, model, v.pipes, page);
            match look(&mut rtl) {
                Ok(()) => Bind::Ours(rtl),
                Err(e) => Bind::Failed("link word unread", e),
            }
        }
        Err((what, e)) => Bind::Failed(what, e),
    }
}
