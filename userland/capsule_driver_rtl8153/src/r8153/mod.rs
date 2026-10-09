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

//! The Realtek RTL8153 and RTL8153B in vendor mode, as Linux r8152 drives
//! them: which adapters, which configuration and chip version, the
//! register layer, the bring-up, the link, and the RX and TX framing.

pub mod bind;
pub mod config;
pub mod enable;
pub mod fail;
pub mod ids;
pub mod link;
mod nic;
pub mod ocp;
pub mod regs;
pub mod rx;
mod transfer;
pub mod tx;
pub mod up;
pub mod version;
mod watch;

pub use bind::bind;
pub use link::Rtl8153;
pub use version::Version;
