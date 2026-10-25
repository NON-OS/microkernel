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

#![no_std]
#![no_main]
/* No outside input may index, overflow or truncate (plan/41): settle is
 * held to the lints its proto crate is. */
#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation
)]

extern crate alloc;

mod chain;
mod drain;
mod handlers;
mod keccak;
mod listing;
mod net;
mod server;
mod settle;

use nonos_libc::{heap_init, mk_exit};

/// The capsule's entry point.
///
/// # Safety
///
/// Only the kernel calls it, once, on a fresh stack; nothing else may.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    server::run();
}
