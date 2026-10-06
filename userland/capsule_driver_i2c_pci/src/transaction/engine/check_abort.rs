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
use crate::constants::{
    ABRT_GCALL_NOACK, ABRT_LOST, ABRT_NOACK_MASK, IC_CLR_TX_ABRT, IC_INTR_TX_ABRT,
    IC_RAW_INTR_STAT, IC_TX_ABRT_SOURCE,
};
use crate::regs::Regs;
use crate::transaction::{TransferError, TransferResult};

/// Report a TX_ABRT the way Linux i2c_dw_handle_tx_abort sorts it: an
/// unacknowledged address or byte is the device saying no (Nack, the probe
/// reads it as "absent"), lost arbitration is a busy bus worth retrying, and
/// anything else (SDA stuck, a protocol violation the core caught) is a bus
/// fault. Reading IC_CLR_TX_ABRT releases the flushed TX FIFO.
pub fn check_abort(regs: Regs, out: &mut TransferResult) -> Result<(), TransferError> {
    if regs.read32(IC_RAW_INTR_STAT) & IC_INTR_TX_ABRT == 0 {
        return Ok(());
    }
    let source = regs.read32(IC_TX_ABRT_SOURCE);
    out.abort_source = source;
    let _ = regs.read32(IC_CLR_TX_ABRT);
    Err(classify_abort(source))
}

fn classify_abort(source: u32) -> TransferError {
    if source & (ABRT_NOACK_MASK | ABRT_GCALL_NOACK) != 0 {
        TransferError::Nack
    } else if source & ABRT_LOST != 0 {
        TransferError::Busy
    } else {
        TransferError::Timeout
    }
}
