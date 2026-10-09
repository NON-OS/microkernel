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

//! The PE Authenticode SHA-256 of an image file: the digest the firmware
//! extends PCR 4 with when it starts a UEFI application, and the digest a
//! Secure Boot signature covers, so signing a loader leaves it unchanged.

mod digest;
mod error;
mod field;
mod pe;

pub use digest::digest;
pub use error::PeError;
pub use pe::MAX_SECTIONS;
