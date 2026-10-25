// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! On an Intel SATA controller, configuration space 90h to 97h as the
//! firmware left it, beside the CAP and PI line: the port mapping and port
//! control registers, MAP and PCS. Read and logged, never written. Where they
//! sit depends on the generation (PCS at 92h on the ICH and older PCHs, 94h
//! with MAP's SPDx at 90h on current SoCs), and the firmware owns them; the
//! line is how a hardware report shows whether one ever leaves an
//! implemented port disabled (HANDOFF/letters/h/03).

use nonos_libc::mk_pci_config_read;

use crate::log::Line;

const INTEL: u16 = 0x8086;

pub(super) fn say_port_map(vendor: u16, device_id: u64, epoch: u64) {
    if vendor != INTEL {
        return;
    }
    let read = |off: u32| mk_pci_config_read(device_id, epoch, off, 4);
    let (lo, hi) = (read(0x90), read(0x94));
    let mut line = Line::new();
    line.text(b"cfg 90h ");
    word(&mut line, lo);
    line.text(b" 94h ");
    word(&mut line, hi);
    line.send();
}

/// The dword read, or `unread` when the broker refused it.
fn word(line: &mut Line, v: i64) {
    match u32::try_from(v) {
        Ok(v) => {
            line.hex(v);
        }
        Err(_) => {
            line.text(b"unread");
        }
    }
}
