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

//! The lock screen's passphrase, as the keyring keeps it: never the
//! passphrase, only an Argon2id tag of it under a random salt, with the
//! count of wrong entries since the last right one.

#![no_std]

extern crate alloc;

/* The kernel's Argon2 names its wipe by this path. */
mod crypto {
    pub mod constant_time {
        pub use crate::wipe::secure_zero;
    }
}

#[path = "../../../src/crypto/util/argon2/mod.rs"]
pub mod argon2;

pub mod backoff;
pub mod blob;
pub mod field;
pub mod record;
pub mod rules;
pub mod verify;
pub mod wipe;
pub mod wire;

pub use argon2::{Argon2Error, Params};
pub use backoff::Gate;
pub use record::{Record, RECORD_LEN, SALT_LEN, TAG_LEN};
