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

//! The listed controllers the namespace declares, each published to the
//! broker once the platform is up (after _INI, so _STA and _CRS answer as
//! they will for the OS). A device the kernel refuses stays unpublished:
//! its driver finds no device, which is the safe outcome.

use super::deps::{deps_unmet, ordered};
use super::say::{found, skipped, unsent};
use super::wire::record;
use crate::boot::{evaluator, path};
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use crate::sys::platform::platform_device;
use nonos_aml::deps::Verdict;
use nonos_aml::load::Aml;
use nonos_aml::platform::DeviceInfo;
use nonos_aml::platform_device::{known, platform_devices};

pub(super) static IDS: &[u8] = include_bytes!("../../data/platform_ids.tsv");

pub fn publish(devices: &[DeviceInfo], aml: &mut Aml, host: &mut CapsuleHost) {
    let (list, dropped) = known(IDS);
    if dropped != 0 {
        Line::new(b"platform ids: ").dec(dropped as u64).str(b" malformed lines dropped").end();
    }
    let f = platform_devices(aml, &mut evaluator(host), devices, &list);
    for (id, why) in &f.skipped {
        skipped(path(aml, *id).as_bytes(), *why);
    }
    for (i, verdict) in ordered(devices, aml, host, &f.devices) {
        let d = &f.devices[i];
        let p = path(aml, d.id);
        let source = list.iter().find(|k| k.id == d.matched).map_or(&b""[..], |k| k.source);
        let unmet = verdict != Verdict::Ready;
        if unmet {
            deps_unmet(p.as_bytes(), verdict);
        }
        match record(d, p.as_bytes(), unmet) {
            Some(r) => found(p.as_bytes(), d, source, platform_device(&r)),
            None => unsent(p.as_bytes(), d),
        }
    }
}
