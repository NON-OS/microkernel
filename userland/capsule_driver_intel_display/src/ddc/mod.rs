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

//! DDC/CI: a monitor's controls over its DDC lines, as VESA's DDC/CI and
//! MCCS standards lay them out (the facts are this driver's reading of
//! them, unverified against the documents). Only the claimed output's own
//! bus is spoken on (`display/ddc_of.rs`), only at the one DDC/CI
//! address, only for the controls the allowlist names, and every reply is
//! checked for its length, its checksum and that it answers what was asked.

pub mod bus;
pub mod caps;
pub mod caps_fetch;
pub mod caps_frame;
mod caps_list;
mod caps_take;
mod caps_walk;
pub mod error;
pub mod frame;
mod fresh;
pub mod pace;
pub mod reply;
pub mod transact;
pub mod transact_write;
pub mod vcp;
pub mod wait;
