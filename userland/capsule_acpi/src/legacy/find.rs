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

//! The legacy devices the namespace declares, read once the platform is up,
//! and whether there is an 8042: the PS/2 driver asks before it touches
//! ports 0x60 and 0x64, and finds no controller where firmware names none.

use crate::boot::{evaluator, path};
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::legacy::{boot_arch, has_8042, legacy_devices, BootArch, Legacy, LegacyKind};
use nonos_aml::load::Aml;
use nonos_aml::platform::DeviceInfo;

#[derive(Default)]
pub struct Legacies {
    pub list: Vec<Legacy>,
    pub boot: Option<BootArch>,
    pub has_8042: bool,
}

fn name(k: LegacyKind) -> &'static [u8] {
    match k {
        LegacyKind::Keyboard => b"keyboard",
        LegacyKind::Mouse => b"mouse",
        LegacyKind::Uart => b"uart",
        LegacyKind::Rtc => b"rtc",
        LegacyKind::Hpet => b"hpet",
        LegacyKind::Pit => b"pit",
    }
}

pub fn find(
    fadt: &[u8],
    devices: &[DeviceInfo],
    aml: &mut Aml,
    host: &mut CapsuleHost,
) -> Legacies {
    let list = legacy_devices(aml, &mut evaluator(host), devices);
    let boot = boot_arch(fadt);
    for l in &list {
        let line =
            Line::new(b"legacy ").str(name(l.kind)).str(b" ").str(path(aml, l.id).as_bytes());
        let line = l.io.iter().fold(line, |s, r| s.str(b" io ").hex(r.base).str(b"+").dec(r.len));
        let line =
            l.memory.iter().fold(line, |s, r| s.str(b" mem ").hex(r.base).str(b"+").hex(r.len));
        l.irqs.iter().fold(line, |s, i| s.str(b" irq ").dec(u64::from(i.gsiv))).end();
    }
    let has = has_8042(boot, &list);
    let declared = list.iter().any(|l| matches!(l.kind, LegacyKind::Keyboard | LegacyKind::Mouse));
    let why: &[u8] = match (has, declared) {
        (false, _) => b"none: no PS/2 device declared and the FADT says no 8042",
        (true, true) => b"declared in the namespace",
        (true, false) => b"the FADT's boot flag alone",
    };
    Line::new(b"8042: ").str(why).end();
    Legacies { list, boot, has_8042: has }
}
