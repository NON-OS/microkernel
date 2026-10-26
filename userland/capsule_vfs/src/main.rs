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

extern crate alloc;

mod blk;
mod protocol;
mod ram;
mod server;
mod store;

use nonos_libc::{heap_init_sized, mk_exit};
use store::budget_size::{for_ram, set, DATA_BYTES_MAX, HEAP_BESIDE};

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    /*
     * The heap holds the clients' budget, sized to the machine, and the
     * staged packages and the server beside it. MkMmap backs all of it at
     * once, so a machine that cannot map the larger one keeps the floor's.
     */
    let wanted = ram::total().map_or(DATA_BYTES_MAX, for_ram);
    let mapped =
        [wanted, DATA_BYTES_MAX].into_iter().find(|b| heap_init_sized(b + HEAP_BESIDE).is_ok());
    match mapped {
        Some(budget) => set(budget),
        None => {
            mk_exit(1);
        }
    }
    server::run();
}
