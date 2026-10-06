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

//! Binding an RTL8153 in Linux's order: the adapter by its IDs
//! (rtl8152_table), the vendor configuration and the chip version
//! (rtl8152_cfgselector_probe reads the version before it configures
//! anything), the bulk pipes to driver.xhci0, SET_CONFIGURATION, then
//! the bring-up.

use nonos_usbnet::{Bind, Bus, Found, Setup};

use super::config::{vendor_config, VendorConfig};
use super::fail::{at, Fail, E_NODEV};
use super::ids::is_rtl8153_adapter;
use super::link::Rtl8153;
use super::ocp::{read_dword, Dev, PLA};
use super::regs::pla::TCR0;
use super::up::bring_up;
use super::version::{version, Version};

pub fn bind<B: Bus>(bus: B, found: &Found) -> Bind<Rtl8153<B>> {
    if !is_rtl8153_adapter(found.info.vendor, found.info.product) {
        return Bind::NotOurs;
    }
    let Some(cfg) = found.configs.iter().find_map(|raw| vendor_config(raw)) else {
        return Bind::Failed("no vendor configuration with bulk endpoints 1 and 2", E_NODEV);
    };
    let mut dev = Dev::new(bus);
    match start(&mut dev, &cfg) {
        Ok((v, mac)) => Bind::Ours(Rtl8153::new(dev, v, mac, cfg.pipes)),
        Err((what, e)) => Bind::Failed(what, e),
    }
}

/// The controller is given the pipes before the device is told to use
/// them (xHCI 1.2, 4.3.5), as for the ECM driver.
fn start<B: Bus>(dev: &mut Dev<B>, cfg: &VendorConfig) -> Result<(Version, [u8; 6]), Fail> {
    let tcr0 = at("chip version unread", read_dword(dev, PLA, TCR0))?;
    let v = version(tcr0).map_err(|what| (what, E_NODEV))?;
    at("bulk pipes refused", dev.bus.configure_bulk(&cfg.pipes))?;
    let config = Setup::set_configuration(cfg.value);
    at("SET_CONFIGURATION refused", dev.bus.control_out(config, &[]))?;
    let mac = bring_up(dev, v)?;
    Ok((v, mac))
}
