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

use super::constants::*;
use super::{PlicError, PlicResult};

pub(super) fn valid_irq(irq: u32) -> PlicResult<()> {
    if irq == 0 || irq >= MAX_INTERRUPTS {
        Err(PlicError::InvalidInterrupt)
    } else {
        Ok(())
    }
}

pub(super) fn context(hart: usize) -> u64 {
    (hart * 2 + 1) as u64
}

pub(super) fn priority(base: u64, irq: u32) -> u64 {
    base + PRIORITY_BASE + (irq as u64 * 4)
}

pub(super) fn pending(base: u64, irq: u32) -> u64 {
    base + PENDING_BASE + ((irq / 32) as u64 * 4)
}

pub(super) fn enable(base: u64, hart: usize, irq: u32) -> u64 {
    base + ENABLE_BASE + (context(hart) * ENABLE_STRIDE) + ((irq / 32) as u64 * 4)
}

pub(super) fn threshold(base: u64, hart: usize) -> u64 {
    base + THRESHOLD_BASE + (context(hart) * CONTEXT_STRIDE)
}

pub(super) fn claim(base: u64, hart: usize) -> u64 {
    base + CLAIM_BASE + (context(hart) * CONTEXT_STRIDE)
}
