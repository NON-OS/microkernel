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

//! Applying the image's own relocations, so it runs at whatever address the
//! loader put it.
//!
//! The kernel is linked as a static position-independent executable at
//! 0x4008_0000. Code reaches everything PC-relative, but a pointer stored in
//! data holds a link-time address, and the linker leaves each one as an
//! R_AARCH64_RELATIVE entry in `.rela.dyn`: write the load bias plus the
//! entry's addend at the entry's offset (AArch64 ELF ABI, IHI 0056, 5.7.10).
//! ld.lld writes nothing at those places beforehand, so this runs on every
//! boot, at the link address too.
//!
//! It runs before any other Rust and before the MMU is on, so it must not
//! reach a static, which may be one of the pointers not yet written, and must
//! not panic: there is nothing yet to report a panic with. Every check runs
//! before the first write, so a refused image is left as it was loaded.

mod apply;
mod entry;
mod rela;
