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

//! The SCI record claimed and bound, and the event ports granted.

use super::state::Sci;
use crate::events_io::PortIo;
use crate::sys::acpi::ports;
use crate::sys::log::Line;
use nonos_aml::events::{enter_acpi_mode, Events, Fadt};
use nonos_libc::{mk_device_claim, mk_device_list, mk_irq_bind, DeviceRecord, IrqBindOut};

/// The kernel's SCI record (src/hardware/broker/acpi/sci.rs).
const CLASS_OTHER: u32 = 0xFFFF;
const PNP_VENDOR: u16 = 0x0001;
const PNP0C08: u16 = 0x0C08;

pub(super) fn claim_and_bind() -> Option<Sci> {
    let mut recs = [DeviceRecord::empty(); 16];
    let n = mk_device_list(CLASS_OTHER, recs.as_mut_ptr(), recs.len() as u64).max(0) as usize;
    let rec =
        recs[..n.min(recs.len())].iter().find(|r| r.vendor == PNP_VENDOR && r.device == PNP0C08)?;
    let epoch = mk_device_claim(rec.device_id);
    if epoch < 0 {
        return None;
    }
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    (mk_irq_bind(rec.device_id, epoch as u64, rec.irq_line as u32, 0, 0, &mut out) >= 0)
        .then_some(Sci { grant: out.grant_id, seq: 0 })
}

/// The ports the event layer reaches: PM1 event and control, the GPE
/// blocks, and the SMI command port's one byte.
pub(super) fn grant_ports(f: &Fadt) -> bool {
    let blocks = [f.pm1a_evt, f.pm1b_evt, f.pm1a_cnt, f.pm1b_cnt, f.gpe0, f.gpe1];
    let ok = blocks.iter().flatten().all(|b| ports(b.port as u64, b.len as u64).is_ok());
    ok && (f.smi_cmd == 0 || ports(f.smi_cmd as u64, 1).is_ok())
}

/// The EC's GPE, served by the capsule's EC driver rather than a method.
pub(super) fn arm_ec(ev: &mut Events, gpe: u16, wake: bool) {
    if ev.arm_native(&mut PortIo, gpe, wake).is_err() {
        Line::new(b"EC GPE ").hex(gpe as u64).str(b" not armed").end();
    }
}

/// How long firmware gets to answer ACPI_ENABLE.
const MODE_WAIT_MS: u64 = 3000;

pub(super) fn enter_mode(f: &Fadt) {
    match enter_acpi_mode(f, &mut PortIo, MODE_WAIT_MS) {
        Ok(true) => Line::new(b"entered ACPI mode").end(),
        Ok(false) => Line::new(b"already in ACPI mode").end(),
        Err(e) => {
            Line::new(b"ACPI mode not entered: ").str(alloc::format!("{e:?}").as_bytes()).end()
        }
    }
}
