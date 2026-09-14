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

//! The per-boot secret, drawn before anything derives from it.
//!
//! First in the sequence because the stack canary, the physical allocator's
//! seed and the memory-proof nonce all read it through `boot_nonce`, and each
//! of them falls back to a compile-time constant when it is unset. Drawing it
//! later would leave whatever ran earlier holding that constant for the life
//! of the boot.
//!
//! It can run this early because it needs nothing but the cycle counter and,
//! where the part has one, RDRAND. No allocation, no paging, nothing to wait
//! for.

use crate::memory::kaslr;
use crate::sys::serial;

pub(super) fn init_boot_entropy() {
    match kaslr::seed_boot_nonce() {
        Ok(hardware) => {
            serial::print(b"[BOOT-ENTROPY] nonce drawn, hardware generator ");
            serial::println(if hardware { b"present" } else { b"absent" });
        }
        Err(_) => {
            /*
             * Not fatal: everything downstream still runs, on the constants
             * it falls back to. Not silent either, because this line is the
             * only warning anyone gets that the canary holds no secret.
             */
            serial::println(b"[BOOT-ENTROPY] WARNING no nonce; canary and seeds are constants");
        }
    }
}
