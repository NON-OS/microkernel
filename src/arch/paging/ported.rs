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

//! Whether the shared VM layer can run on this architecture. It needs a
//! page-descriptor backend, a direct map at `DIRECTMAP_BASE` and the root and
//! TLB facades. x86_64 and aarch64 have all of them. riscv64 has none: its
//! boot tables are Sv39, which cannot address the direct map, and it has no
//! page-descriptor backend (`descriptor/mod.rs` mounts none for it). So the boot
//! stops here and says why, rather than fault on the first write through the
//! direct map.
pub fn require_vm_port() {
    #[cfg(target_arch = "riscv64")]
    {
        crate::sys::serial::print(b"[VM] riscv64: the VM layer is not ported ");
        crate::sys::serial::println(b"(no Sv48 direct map, no page descriptors); stopping");
        crate::arch::halt_loop()
    }
}
