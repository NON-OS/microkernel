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

//! Whether a capsule may bind a line: not one the kernel keeps, not one
//! already granted, not one of a function already sending messages.

use super::super::records;
use super::super::types::IrqBindError;

pub(super) fn line_free(gsi: u32, device_id: u64) -> Result<(), IrqBindError> {
    // Ahead of the grant records: those only describe lines other capsules
    // hold. The kernel programs its own through a different module, so a line
    // it listens on has to be refused here or a capsule reprograms it.
    if super::super::reserved::is_reserved(gsi) {
        return Err(IrqBindError::ReservedGsi);
    }
    // A function sending messages does not assert INTx (PCI Local Bus 3.0,
    // 6.8), so the line would be bound and never fire.
    if records::vector_for_gsi(gsi).is_some() || records::has_message_grant(device_id) {
        return Err(IrqBindError::AlreadyBound);
    }
    Ok(())
}
