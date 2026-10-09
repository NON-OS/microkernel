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

//! What the vfs block layer and ramfs take from `nonos_libc`, for the host.
//!
//! The store calls answer as the kernel's `MkStoreRead` and `MkStoreWrite`
//! do (`src/syscall/microkernel/store_*.rs`): the same window, the same
//! request limits, the same errno for each refusal, and ENODEV on a read
//! when no disk carries NONOS. Behind them is a disk in memory, one per test
//! thread, which a test can take away, make fault, or cut the power to after
//! a given number of sectors, so a proof can stop a write at every sector it
//! puts down and read what the next boot would find. The crypto calls are
//! ramfs's, with the kernel's sizes and refusals.

mod clock;
mod crypto;
mod debug;
pub mod disk;
mod store;

pub use clock::mk_uptime_ms;
pub use crypto::{crypto_decrypt, crypto_encrypt, crypto_random};
pub use debug::mk_debug;
pub use store::{mk_store_read, mk_store_write};
