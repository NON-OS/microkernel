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

//! Finding the slot BAR and mapping its register window.

use nonos_libc::{mk_mmio_map, mk_pci_config_read, MmioMapOut, BAR_KIND_MMIO};

use super::super::super::env::Log;
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::pci::{first_bar, PCI_SLOT_INFO};
use super::super::super::sdhci::regs::SLOT_WINDOW;
use super::super::super::text::Line;
use super::super::discover::Found;
use super::super::env::{MmioWindow, SerialLog};
use super::claim::Claim;
use super::sizes::slot_length;

/// Read the Slot Information byte, check the slot BAR and map its window;
/// the grant is kept in `claim`.
pub(super) fn map_slot(
    log: &SerialLog,
    dev: &Found,
    epoch: u64,
    claim: &mut Claim,
) -> EmmcResult<MmioWindow> {
    // An ACPI host has no Slot Information byte: its first window is slot 0.
    let info =
        if dev.acpi { -1 } else { mk_pci_config_read(dev.device_id, epoch, PCI_SLOT_INFO, 1) };
    let bar = if info < 0 { 0 } else { first_bar(info as u8) };
    let b = dev.bars[bar as usize];
    let length = slot_length(b.size, SLOT_WINDOW);
    let usable = bar < dev.bar_count && b.kind == BAR_KIND_MMIO;
    let Some(length) = length.filter(|_| usable) else {
        log.line(
            Line::new()
                .s(b"slot BAR ")
                .dec(bar as u64)
                .s(b" unusable, size ")
                .hex(b.size)
                .as_bytes(),
        );
        return Err(EmmcError::Window);
    };
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let r = mk_mmio_map(dev.device_id, epoch, bar as u32, 0, 0, length, &mut out);
    if r < 0 {
        log.line(Line::new().s(b"MMIO map refused (").dec(r.unsigned_abs()).s(b")").as_bytes());
        return Err(EmmcError::Broker(r));
    }
    claim.mmio_grant = Some(out.grant_id);
    // A window the broker cut short of one slot (a protected page) is no use.
    if out.length < SLOT_WINDOW {
        return Err(EmmcError::Window);
    }
    // SAFETY: the grant just made covers `out.length` >= 256 bytes and lives in
    // `claim`, which the returned `Opened` holds past the last access.
    let io = unsafe { MmioWindow::new(out.user_va) };
    Ok(io)
}
