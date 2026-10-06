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

//! The installer's package authentication, included from the capsule. The
//! RSA call is the one part left out: it is an IPC to the crypto service, and
//! a test hands the same check in as a closure.

#[path = "../../../../capsule_linux/src/linux/install/auth/base64.rs"]
pub mod base64;

#[path = "../../../../capsule_linux/src/linux/install/auth/checksum.rs"]
pub mod checksum;

#[path = "../../../../capsule_linux/src/linux/install/auth/digest.rs"]
pub mod digest;

#[path = "../../../../capsule_linux/src/linux/install/auth/keys.rs"]
pub mod keys;

#[path = "../../../../capsule_linux/src/linux/install/auth/package.rs"]
pub mod package;

#[path = "../../../../capsule_linux/src/linux/install/auth/pkginfo.rs"]
pub mod pkginfo;

#[path = "../../../../capsule_linux/src/linux/install/auth/signature.rs"]
pub mod signature;

#[path = "../../../../capsule_linux/src/linux/install/auth/verified.rs"]
pub mod verified;

pub use checksum::parse as checksum;
