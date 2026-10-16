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
use crate::error::{XhciError, XhciResult};
use crate::regs::cap::max_slots;

/// A controller with no device slots cannot serve a device. One without
/// 64-bit addressing (HCCPARAMS1.AC64 clear) is served: its DMA pool keeps
/// every buffer below 4 GiB instead (`DmaPool::with_ac64`). Refusing it, as
/// this once did, turned away working 32-bit controllers.
pub fn refuse_unsupported(mmio_base: u64) -> XhciResult<()> {
    if max_slots(mmio_base) == 0 {
        return Err(XhciError::ControllerUnsupported);
    }
    Ok(())
}
