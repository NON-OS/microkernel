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

//! One `[ACPI] pci host` line per host bridge and one `[ACPI] pci label`
//! line per labelled device.

use super::state::{Bdf, Host};
use crate::boot::path;
use crate::sys::log::Line;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::pci_dsm::Label;

pub fn say_host(aml: &Aml, h: &Host) {
    let l = Line::new(b"pci host ").str(path(aml, h.id).as_bytes()).str(b": buses ");
    let l = l.hex(u64::from(h.buses.0)).str(b"-").hex(u64::from(h.buses.1));
    let l = match h.facts.preserve_boot_config {
        Some(true) => l.str(b"; boot configuration kept"),
        Some(false) => l.str(b"; boot configuration may be redone"),
        None => l.str(b"; boot configuration unsaid"),
    };
    match h.facts.readiness {
        Some(r) => {
            let l = l.str(b"; ready after reset ").dec(u64::from(r.reset_us)).str(b"us, link ");
            let l = l.dec(u64::from(r.link_up_us)).str(b"us, FLR ").dec(u64::from(r.flr_us));
            l.str(b"us, D3hot->D0 ").dec(u64::from(r.d3hot_to_d0_us)).str(b"us").end();
        }
        None => l.end(),
    }
}

pub fn say_label(aml: &Aml, id: Id, f: Bdf, label: &Label) {
    let l = Line::new(b"pci label ").hex(u64::from(f.1)).str(b":").hex(u64::from(f.2));
    let l = l.str(b".").dec(u64::from(f.3)).str(b" ").str(path(aml, id).as_bytes());
    l.str(b" #").dec(label.instance).str(b" \"").str(&label.text).str(b"\"").end();
}
