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

use super::super::types::{IrqBindError, IrqBindRequest, FLAGS_KNOWN};

/// Validate the INTx branch. The MSI-X bind path must not call
/// this; the regular `bind_intx` flow does.
pub(in super::super) fn validate_intx_request(
    req: &IrqBindRequest,
    irq_pin: u8,
    irq_line: u8,
) -> Result<(), IrqBindError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.flags != 0 {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.vector_count != 0 {
        return Err(IrqBindError::BadVectorCount);
    }
    if irq_pin == 0 || irq_line == 0xFF {
        return Err(IrqBindError::NotIntx);
    }
    if req.irq_source != irq_line as u32 {
        return Err(IrqBindError::NotDeviceIrq);
    }
    Ok(())
}
