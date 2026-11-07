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

//! The lock screen's passphrase verifier: held in memory, checked here and
//! nowhere else, and handed out only as the blob the shell keeps on disk.

mod blob;
mod handlers;
pub mod keeper;

pub use handlers::{handle, Lock};

/// The keyring's heap beside the Argon2id work area the verifier needs.
pub const HEAP_WITH_LOCK: usize =
    nonos_libc::HEAP_SIZE + nonos_lock::record::MAX_M_KIB as usize * 1024 + 4 * 1024 * 1024;
