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

//! The driver's errors.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtsxError {
    Claim,
    MapBar,
    PciCommand,
    DmaMap,
    /// A DMA buffer the broker placed above 4 GiB; the reader takes 32-bit
    /// addresses only (rtsx_pci_probe sets a 32-bit DMA mask).
    DmaAbove4G,
    RegisterTimeout,
    RegisterMismatch,
    PhyTimeout,
    CmdOverflow,
    CmdTimeout,
    CmdFailed,
    Gone,
    ResponseShort,
    ResponseStartBits,
    ResponseCrc7,
    CardStatus,
    NotSd,
    PowerUpTimeout,
    UnknownCsd,
    ClockRange,
    DataTimeout,
    DataFailed,
}

pub type Result<T> = core::result::Result<T, RtsxError>;
