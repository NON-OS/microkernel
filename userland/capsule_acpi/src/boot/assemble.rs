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

//! The capsule's state as the first pass needs it: the namespace, the
//! devices, serial and legacy facts. What the rest of start fills in
//! (events, batteries, buttons, cooling, processors, backlight, hotkeys,
//! hardware errors, Type-C) starts empty.

use super::state::Acpi;
use crate::host::CapsuleHost;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::platform::{DeviceInfo, Route};

pub struct Parts {
    pub aml: Aml,
    pub host: CapsuleHost,
    pub ec: Option<crate::ec::Ec>,
    pub devices: Vec<DeviceInfo>,
    pub routes: Vec<(Id, Vec<Route>)>,
    pub serial: crate::serial::Serial,
    pub legacy: crate::legacy::Legacies,
    pub smbios: Option<nonos_smbios::identity::Identity>,
}

pub fn assemble(p: Parts) -> Acpi {
    let Parts { aml, host, ec, devices, routes, serial, legacy, smbios } = p;
    Acpi {
        aml,
        host,
        events: None,
        sci: None,
        ec,
        power: Default::default(),
        buttons: Default::default(),
        cooling: Default::default(),
        cpus: Default::default(),
        dpower: Default::default(),
        hidden: Default::default(),
        hotplug: Default::default(),
        pci_firmware: Default::default(),
        backlight: Default::default(),
        hotkeys: Default::default(),
        serial,
        legacy,
        apei: Default::default(),
        smbios,
        typec: None,
        aei: Default::default(),
        devices,
        routes,
        scis: 0,
        gpe_runs: 0,
    }
}
