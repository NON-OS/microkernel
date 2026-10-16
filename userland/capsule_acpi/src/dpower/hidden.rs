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

//! At boot, the devices firmware describes under a PCI root but left so
//! far off that enumeration saw nothing: present and enabled by _STA, with
//! _PS0 or _PR0 to turn them on, and an _ADR that names a function reading
//! all ones. Each gets its D0 and one kernel probe; nothing else is woken.

use crate::boot::{evaluator, path, Acpi};
use crate::sys::acpi_pci::pci_read;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use crate::sys::pci_probe::probe_function;
use alloc::vec::Vec;
use nonos_aml::device_power::{power_managed, reveal, Revealed};
use nonos_aml::namespace::Id;
use nonos_aml::platform::{device_pci_path, function_of, ENABLED, PRESENT, SECONDARY_BUS};

/// How long drivers have to claim what was revealed before it goes back.
const SETTLE_MS: u64 = 60_000;

pub type Bdf = (u16, u8, u8, u8);

/// Only a real PCI-to-PCI bridge (header type 1) gives a secondary bus; a
/// USB controller's ports carry _ADRs too, and are not functions.
pub fn bridge_bus(f: Bdf) -> Option<u8> {
    let header = pci_read(f, 0x0E, 1).ok()? & 0x7F;
    (header == 1).then(|| pci_read(f, SECONDARY_BUS, 1).ok().map(|v| v as u8)).flatten()
}

fn missing(acpi: &mut Acpi, dev: Id) -> Option<Bdf> {
    let path = device_pci_path(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev)?;
    let f = function_of(&path, &mut bridge_bus)?;
    pci_read(f, 0, 2).map_or(true, |v| v == 0xFFFF).then_some(f)
}

pub fn reveal_hidden(acpi: &mut Acpi) {
    let want = PRESENT | ENABLED;
    let fitted = |d: &&nonos_aml::platform::DeviceInfo| d.sta & want == want && d.adr.is_some();
    let devs: Vec<Id> = acpi.devices.iter().filter(fitted).map(|d| d.id).collect();
    let devs: Vec<Id> = devs.into_iter().filter(|d| power_managed(&acpi.aml, *d)).collect();
    for dev in devs {
        let Some(f) = missing(acpi, dev) else { continue };
        let Acpi { aml, host, dpower, cooling, .. } = acpi;
        let mut probe = || f.0 == 0 && probe_function(f.1, f.2, f.3).is_ok();
        let r = reveal(dpower, &mut cooling.pool, aml, &mut evaluator(host), dev, &mut probe);
        let l = Line::new(b"hidden ").str(path(aml, dev).as_bytes());
        match r {
            Revealed::Answered => {
                l.str(b": answered after _PS0").end();
                acpi.hidden.answered.push((dev, f));
            }
            Revealed::Silent => l.str(b": no answer after _PS0, back to D3cold").end(),
            Revealed::Failed(_) => l.str(b": _PS0 failed").end(),
        }
    }
    acpi.hidden.settle_at = now_ms().saturating_add(SETTLE_MS);
}
