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

mod abi;
mod app;
mod assemble;
mod job;
mod market;

use nonos_app_skeleton::run;

/// # Safety
/// The kernel enters here once, on a fresh stack, before anything else in
/// this capsule has run.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    /* A receipt takes a few KiB: the default heap, never the prover's floor. */
    if job::is_receipt() {
        job::serve(nonos_libc::heap_init().is_ok(), job::bound());
    }
    /* The floor is held, or refused, before the first allocation. */
    let memory = app::claim();
    // The store's headless job serves one PROVE and exits; no window. Short
    // of the floor it still answers, ENOMEM, from the default heap.
    if job::is_job() {
        let _ = nonos_libc::heap_init();
        job::serve(memory == app::Memory::Held, job::bound());
    }
    run(move || app::Prove::new(memory))
}
