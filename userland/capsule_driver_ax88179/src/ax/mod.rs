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

//! The ASIX AX88179 and AX88178A: finding the chip, bringing it up through
//! its vendor register access, its receive and send framing, and its link.

pub mod bind;
pub mod bring_up;
pub mod claim;
pub mod counts;
pub mod follow;
pub mod function;
pub mod ids;
pub mod link;
pub mod medium;
mod nic;
pub mod plan;
mod receive;
pub mod regs;
pub mod rx_header;
pub mod rx_span;
pub mod status;
pub mod step;
mod transfer;
pub mod tx_header;
pub mod vendor;
pub mod watch;

pub use bind::bind;
