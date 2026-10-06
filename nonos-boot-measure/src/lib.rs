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

//! The bootloader as the firmware measured it, checked from outside.
//!
//! The firmware hashes every UEFI application it starts with the PE Authenticode
//! SHA-256, extends PCR 4 with it and appends an event to the TCG log. The log
//! replayed must give the live PCR 4, and then its last application event is
//! what ran last before the kernel: the bootloader, or whatever was started
//! after it, which then fails the bootloader's enrollment. The boot-root record
//! says which bootloader tree the release vouches for, at which epoch.
//!
//! Every length here comes from the bytes being read, so every one is bounded
//! before it is used, and nothing in this crate panics on any input.

#![cfg_attr(not(test), no_std)]

pub mod authenticode;
pub mod gate;
pub mod record;
pub mod tcg;

#[cfg(test)]
mod tests;
