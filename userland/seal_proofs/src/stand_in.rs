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

//! What MkSealRead, MkSealPeek and MkSealDrop reach beyond the seal: the
//! device-secret gate, held per test thread, and the user copies, over this
//! process's own memory, with address 0 the one that faults.

use std::cell::Cell;

thread_local! {
    pub static SECRET: Cell<bool> = const { Cell::new(true) };
}

pub(crate) fn device_secret_caller() -> bool {
    SECRET.with(Cell::get)
}

pub(crate) mod usercopy {
    pub(crate) fn copy_from_user(ptr: u64, out: &mut [u8]) -> Result<(), ()> {
        if ptr == 0 {
            return Err(());
        }
        // SAFETY: a test passes the address of a live buffer at least out.len() long.
        out.copy_from_slice(unsafe { core::slice::from_raw_parts(ptr as *const u8, out.len()) });
        Ok(())
    }

    pub(crate) fn write_user_bytes(ptr: u64, bytes: &[u8]) -> Result<(), ()> {
        if ptr == 0 {
            return Err(());
        }
        // SAFETY: a test passes the address of a live buffer at least bytes.len() long.
        unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, bytes.len()) };
        Ok(())
    }
}
