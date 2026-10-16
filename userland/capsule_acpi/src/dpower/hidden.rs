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

use super::ready::{bridge_bus, wait_ready, Bdf};
use crate::boot::{evaluator, path, Acpi};
use crate::sys::acpi_pci::pci_read;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use crate::sys::pci_probe::probe_function;
use alloc::vec::Vec;
use nonos_aml::device_power::{power_managed, reveal, Revealed};
use nonos_aml::namespace::Id;
use nonos_aml::pci_dsm::Wake;
use nonos_aml::platform::{device_pci_path, function_of, ENABLED, PRESENT};

/// How long drivers have to claim what was revealed before it goes back.
const SETTLE_MS: u64 = 60_000;

fn missing(acpi: &mut Acpi, dev: Id) -> Option<Bdf> {
    let path = device_pci_path(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev)?;
    let f = function_of(&path, &mut bridge_bus)?;
    pci_read(f, 0, 2).map_or(true, |v| v == 0xFFFF).then_some(f)
}

/// Each hidden function not yet tried, until `until`; whether all were.
pub fn reveal_hidden(acpi: &mut Acpi, until: u64) -> bool {
    let want = PRESENT | ENABLED;
    let fitted = |d: &&nonos_aml::platform::DeviceInfo| d.sta & want == want && d.adr.is_some();
    let devs: Vec<Id> = acpi.devices.iter().filter(fitted).map(|d| d.id).collect();
    let fresh = |d: &Id| power_managed(&acpi.aml, *d) && !acpi.hidden.tried.contains(d);
    let devs: Vec<Id> = devs.into_iter().filter(fresh).collect();
    for dev in devs {
        if now_ms() >= until {
            return false;
        }
        acpi.hidden.tried.push(dev);
        let Some(f) = missing(acpi, dev) else { continue };
        let Acpi { aml, host, dpower, cooling, pci_firmware, .. } = acpi;
        // Up from D3cold it has been through a reset: let it recover first.
        let mut probe = || {
            let _ = wait_ready(pci_firmware, f, Wake::FromD3Cold);
            f.0 == 0 && probe_function(f.1, f.2, f.3).is_ok()
        };
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
    true
}
