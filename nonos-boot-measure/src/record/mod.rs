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

//! The boot-root record, `\EFI\nonos\boot_root.approval`: which bootloader tree
//! the release vouches for, at which epoch, signed with the release's P-256
//! policy key, the one that approves kernels for the device secret.
//!
//!   0   32  the bootloader tree's root, four canonical little-endian words
//!   32   8  the epoch, u64 little-endian: the release's rollback index
//!   40  32  ECDSA r, big-endian
//!   72  32  ECDSA s, big-endian
//!
//! The signature is over `SHA-256("NONOS-BOOT-ROOT-v1" || root || epoch)`. The
//! epoch is signed so an old root cannot be replayed once the TPM's
//! anti-rollback floor has moved past it.

mod check;
mod error;
mod parse;

pub use check::{check, message};
pub use error::RecordError;
pub use parse::{parse, Record};

pub const RECORD_LEN: usize = 104;
pub const DOMAIN: &[u8] = b"NONOS-BOOT-ROOT-v1";
/// The Goldilocks modulus: a root word at or above it is not a field element.
pub const P: u64 = 0xFFFF_FFFF_0000_0001;
