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

//! Paging is up: the kernel heap's second region, before anything a process
//! does can grow the heap. Every page a process maps costs a record in the
//! kernel heap (memory/heap/manager/init.rs), and on the bootstrap heap alone
//! about 3.5 GiB mapped across the machine filled it.

pub(super) fn init_heap_region() {
    crate::sys::bench::mark(b"vm_ready");
    let mut l = crate::sys::serial::Line::new();
    match crate::memory::heap::init() {
        Ok(size) => {
            l.str(b"[HEAP] second region ").dec((size / (1024 * 1024)) as u64).str(b" MiB").end();
        }
        Err(_) => l.str(b"[HEAP] no second region: the bootstrap heap alone").end(),
    }
}
