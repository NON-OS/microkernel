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

//! The grant records and live slots for an MSI-X run the device was just
//! programmed with, one per table entry.

extern crate alloc;

use alloc::vec::Vec;

use super::super::grant::{IrqGrant, IrqGrantKind, NO_LINE};
use super::super::records;
use super::super::slots;
use super::super::types::{IrqBindRequest, IrqBindResult};

pub(super) fn record_run(
    pid: u32,
    req: &IrqBindRequest,
    epoch: u64,
    base_slot: usize,
    base_vector: u8,
    irtes: &[u16],
) -> IrqBindResult {
    let n = irtes.len();
    let base_grant = records::allocate_id_run(n as u64);
    let mut new_records: Vec<IrqGrant> = Vec::with_capacity(n);
    for i in 0..n {
        new_records.push(IrqGrant {
            grant_id: base_grant + i as u64,
            pid,
            device_id: req.device_id,
            claim_epoch: epoch,
            irq_source: NO_LINE,
            vector: base_vector + i as u8,
            flags: req.flags,
            kind: IrqGrantKind::Msix,
            device_vector: i as u16,
            irte: irtes[i],
        });
    }
    records::insert_many(&new_records);

    for i in 0..n {
        slots::activate(base_slot + i, base_grant + i as u64, NO_LINE);
    }

    IrqBindResult { grant_id: base_grant, vector: base_vector }
}
