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

//! The one AES-128 in userland, and the two counter widths the onion
//! transports need.
//!
//! In a crate rather than behind a syscall. A bare stream cipher handed to
//! every capsule is a footgun: reuse a key and counter once and the XOR of two
//! plaintexts falls out. Both callers derive a fresh key per hop per
//! direction, so the constraint is theirs to keep and the kernel does not grow
//! to carry it.
//!
//! Two counter types, not one, because the two protocols genuinely differ and
//! the difference is silent. Sphinx counts in the low 64 bits behind a nonce
//! prefix; a Tor relay counts across all 128 bits from zero. The two agree for
//! the first 2^64 blocks and are still different ciphers, so the width is
//! named in the type and neither caller can pick it by accident.

#![no_std]

mod ctr128;
mod ctr64;
mod encrypt_block;
mod key_schedule;
mod mix_columns;
mod shift_rows;
mod sub_byte;
mod types;
mod xtime;

pub use ctr128::Ctr128Be;
pub use ctr64::Ctr64Be;
pub use types::{Aes128, BLOCK_BYTES, KEY_BYTES};
