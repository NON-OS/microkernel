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

//! The bring-up's stages, each needing what the one before left:
//!
//! - Reset: the part quieted, then reset, so nothing firmware or a previous
//!   OS left running survives. AUTOC is read first.
//! - Mode: the 82599's link mode put back where the reset changed it.
//! - Own: the port taken from manageability firmware.
//! - Address: a drawn station address and clean filters, before anything
//!   can receive.
//! - Settings: the MAC-wide receive and transmit settings the rings rely on.
//! - TxRing, RxRing: each queue given its ring and enabled.
//! - Receiver: the receiver on, last, once there is a ring to receive into.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    Reset,
    Mode,
    Own,
    Address,
    Settings,
    TxRing,
    RxRing,
    Receiver,
    Up,
}

impl Stage {
    pub fn next(self) -> Stage {
        match self {
            Stage::Reset => Stage::Mode,
            Stage::Mode => Stage::Own,
            Stage::Own => Stage::Address,
            Stage::Address => Stage::Settings,
            Stage::Settings => Stage::TxRing,
            Stage::TxRing => Stage::RxRing,
            Stage::RxRing => Stage::Receiver,
            Stage::Receiver | Stage::Up => Stage::Up,
        }
    }
}
