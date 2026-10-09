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

//! The calls nonos_tls makes into nonos_userland_libc, answered on the host.
//!
//! Crypto calls are framed the way the kernel frames them and served by the
//! crypto capsule's real dispatch. Each call is counted per thread, so a test
//! can say how many pool round trips a step of the handshake costs.

extern crate alloc;

#[path = "../../../capsule_crypto/src/protocol/mod.rs"]
pub mod protocol;
pub mod server;

mod aead;
mod count;
mod crypto;
mod ipc;
mod serve;
mod time;

pub use aead::{crypto_decrypt_aad, crypto_encrypt_aad};
pub use count::{counts, reset, Counts};
pub use crypto::{crypto_hash, crypto_hmac_sha256, crypto_random};
pub use crypto::{crypto_x25519_public, crypto_x25519_shared};
pub use ipc::{mk_ipc_call, mk_service_lookup, POOL_PORT};
pub use time::{mk_time_millis, mk_time_rtc, mk_uptime_ms, mk_yield, RtcTime};
