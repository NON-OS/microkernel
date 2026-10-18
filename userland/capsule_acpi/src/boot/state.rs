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

//! Everything the capsule holds once it is up.

use crate::backlight::Backlight;
use crate::buttons::Buttons;
use crate::cooling::Cooling;
use crate::cpu::Cpus;
use crate::ec::Ec;
use crate::host::CapsuleHost;
use crate::hotkeys::Hotkeys;
use crate::power::Power;
use crate::serial::Serial;
use alloc::vec::Vec;
use nonos_aml::device_power::DevicePower;
use nonos_aml::events::Events;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::platform::{DeviceInfo, Route};

pub struct Sci {
    pub grant: u64,
    pub seq: u64,
}

pub struct Acpi {
    pub aml: Aml,
    pub host: CapsuleHost,
    pub events: Option<Events>,
    pub sci: Option<Sci>,
    pub ec: Option<Ec>,
    pub power: Power,
    pub buttons: Buttons,
    pub cooling: Cooling,
    pub cpus: Cpus,
    /// The D-states drivers asked for; the pool they share is `cooling`'s.
    pub dpower: DevicePower,
    /// Functions woken by _PS0 at boot, waiting on a driver's claim.
    pub hidden: crate::dpower::Hidden,
    /// Devices that come and go: what each stood for, ejects in flight.
    pub hotplug: crate::hotplug::Hotplug,
    /// What the PCI `_DSM` says of each host bridge and labelled device.
    pub pci_firmware: crate::pci_firmware::PciFirmware,
    pub backlight: Backlight,
    pub hotkeys: Hotkeys,
    pub serial: Serial,
    pub legacy: crate::legacy::Legacies,
    pub apei: crate::apei::Apei,
    pub smbios: Option<nonos_smbios::identity::Identity>,
    pub typec: Option<crate::typec::TypeC>,
    /// GPIO controllers' event pins and rates, filled as drivers ask.
    pub aei: crate::aei::GpioEvents,
    pub devices: Vec<DeviceInfo>,
    pub routes: Vec<(Id, Vec<Route>)>,
    /// SCIs served and GPE methods run, for the status reply.
    pub scis: u64,
    pub gpe_runs: u64,
}
