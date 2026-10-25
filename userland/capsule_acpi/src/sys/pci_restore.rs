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

//! MkAcpiRestoreFunction ("MARF", the maintainers' kernel half): once a claimed
//! PCI function is in D0 again, the kernel puts back the BARs it lost in
//! D3, as at its claim (L62). Until the kernel offers it, D3hot without
//! No_Soft_Reset stays refused, so whether it is there is asked once.

use super::acpi::{ok, tag4, Errno};
use core::sync::atomic::{AtomicU8, Ordering};
use nonos_libc::mk_syscall_raw;

const RESTORE: i64 = tag4(b"MARF");
const ENOSYS: Errno = -38;
/// 0 not asked yet, 1 there, 2 not.
static PRESENT: AtomicU8 = AtomicU8::new(0);

pub fn restore(device_id: u64) -> Result<(), Errno> {
    ok(mk_syscall_raw(RESTORE, [device_id, 0, 0, 0, 0, 0])).map(|_| ())
}

/// Whether the kernel has the call: device id 0 names no function, so any
/// answer but ENOSYS says it is there.
pub fn restore_present() -> bool {
    match PRESENT.load(Ordering::Relaxed) {
        1 => true,
        2 => false,
        _ => {
            let there = restore(0) != Err(ENOSYS);
            PRESENT.store(if there { 1 } else { 2 }, Ordering::Relaxed);
            there
        }
    }
}
