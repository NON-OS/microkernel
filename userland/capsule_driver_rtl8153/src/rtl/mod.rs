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

//! The RTL8153 family in its vendor configuration: which adapters, which
//! chips, the register path over endpoint 0, the bring-up as tables of
//! register steps, and the framing of the bulk pipes.
//!
//! Behaviour read from Linux v6.12 drivers/net/usb/r8152.c (as read, not
//! copied: the steps are written here as data, under this driver's names);
//! Realtek publishes no programming reference for the chip's vendor mode.

pub mod access;
pub mod adapters;
pub mod autoneg;
pub mod bind;
pub mod bounds;
pub mod bring_up;
pub mod claim;
pub mod common;
pub mod config;
pub mod counts;
pub mod hand_off;
pub mod link_down;
pub mod link_up;
pub mod link_word;
pub mod model;
pub mod nic;
pub mod ocp;
pub mod op;
pub mod pause;
pub mod phy;
pub mod receive;
pub mod reg;
pub mod regs;
pub mod run;
pub mod rx;
pub mod start;
pub mod start_tail;
pub mod state;
pub mod station;
pub mod tx;
pub mod usb_speed;
pub mod version;
pub mod wake;
pub mod watch;

pub use bind::bind;
pub use claim::Claim;
pub use pause::Pause;
