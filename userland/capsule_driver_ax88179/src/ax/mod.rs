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

//! ASIX AX88179 and AX88178A: matching, register access, bring-up, the
//! link on the clock, and the vendor framing of both bulk pipes.

pub mod access;
mod autoneg;
pub mod bind;
pub mod bits;
pub mod bulkin;
mod eee;
pub mod function;
pub mod link;
mod link_poll;
mod link_reset;
pub mod medium;
pub mod phy_regs;
mod power;
pub mod products;
mod receive;
pub mod regs;
pub mod rx;
pub mod station;
mod transfer;
pub mod tx;

pub use bind::bind;
