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
//!   OS left running survives, and the NVM's defaults are loaded.
//! - Port: the NVM has now said what the port is; only copper goes on.
//! - Address: a drawn station address and clean filters, before anything
//!   can receive.
//! - Link: the MAC told to take what the PHY resolves.
//! - Phy: the PHY powered and negotiating.
//! - Engines: the transmit and receive engines on, both queues still off.
//! - TxRing, RxRing: each queue given its ring and enabled.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    Reset,
    Port,
    Address,
    Link,
    Phy,
    Engines,
    TxRing,
    RxRing,
    Up,
}

impl Stage {
    pub fn next(self) -> Stage {
        match self {
            Stage::Reset => Stage::Port,
            Stage::Port => Stage::Address,
            Stage::Address => Stage::Link,
            Stage::Link => Stage::Phy,
            Stage::Phy => Stage::Engines,
            Stage::Engines => Stage::TxRing,
            Stage::TxRing => Stage::RxRing,
            Stage::RxRing | Stage::Up => Stage::Up,
        }
    }
}
