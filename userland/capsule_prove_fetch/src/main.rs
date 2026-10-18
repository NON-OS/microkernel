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

//! nonos.prove.fetch: carries kind-1 proof containers to and from
//! the proof relay for app.store and market.index, over Nym or Anyone and
//! never directly. It never holds DeviceSecret and never sees a witness.

#![no_std]
#![no_main]

extern crate alloc;

mod callers;
mod caps;
mod carry;
mod desk;
mod exchange;
mod io;
mod serve;

use nonos_libc::{heap_init, mk_exit};

/// # Safety
/// The kernel enters here once, on a fresh stack, before anything else in
/// this capsule has run.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    serve::serve()
}
