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
//! Mode setting for Intel's Gen11 and Gen12 (Xe-LP) display engines.
//!
//! Every register and every step is taken from Intel's Tiger Lake
//! Programmer's Reference Manual, cited by volume and page beside it:
//! "Vol 2c" is IHD-OS-TGL-Vol 2c-12.21 (registers), "Vol 12" is
//! IHD-OS-TGL-Vol 12-12.21 (the display engine and its sequences).
//! Nothing here has run on a machine yet; it is held to those pages by
//! intel_xelp_display_proofs, over a model of the registers.

#![no_std]

pub mod ddi;
pub mod phy;
pub mod pll;
pub mod regs;
pub mod timing;
pub mod transcoder;
pub mod wait;
