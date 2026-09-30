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

//! The kernel's ChaCha20-Poly1305 as it stood before the 64-bit Poly1305
//! and the register-held ChaCha20 state (git HEAD 2489b7041,
//! src/crypto/symmetric/chacha20poly1305/), kept here only as the
//! reference the differential tests compare the shipping code against.
//! Same arithmetic, reflowed to fit the file limit.

mod aead;
mod chacha20;
mod poly_block;
mod poly_final;
mod poly_new;
mod poly_update;

pub use aead::{aead_encrypt, poly1305_mac};
pub use chacha20::chacha20_block;
