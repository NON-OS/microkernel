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

//! What BIPR says about a command buffer or a DMA run, read the way
//! rtsx_pci_isr reads it: a failure or a dropped link wins over success.

use crate::regs::host::{DELINK_INT, TRANS_FAIL_INT, TRANS_OK_INT};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Pending,
    Ok,
    Fail,
    /// All ones: the reader no longer answers on the bus.
    Gone,
}

pub const fn classify(bipr: u32) -> Outcome {
    if bipr == u32::MAX {
        Outcome::Gone
    } else if bipr & (TRANS_FAIL_INT | DELINK_INT) != 0 {
        Outcome::Fail
    } else if bipr & TRANS_OK_INT != 0 {
        Outcome::Ok
    } else {
        Outcome::Pending
    }
}
