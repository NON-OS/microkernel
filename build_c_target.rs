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

//! The C target and flags the kernel's assembly is built with, per architecture.

/// The clang target triple and any extra flags the freestanding C in this tree
/// needs for `arch`. Every C source here is compiled for the kernel rather than
/// for the machine doing the build, so the triple follows the cargo target.
/// `None` where there is no kernel C for that architecture.
///
/// The two x86_64 flags have no counterpart elsewhere. Kernel code cannot use the
/// red zone below the stack pointer, because an interrupt frame would land in it,
/// and the kernel code model places the image in the top 2GB. aarch64 addresses
/// the whole space uniformly and needs neither.
pub(crate) fn c_target(arch: &str) -> Option<(&'static str, &'static [&'static str])> {
    match arch {
        // No vector registers: kernel code runs before a thread's are saved.
        "x86_64" => Some((
            "x86_64-unknown-none-elf",
            &["-mno-red-zone", "-mcmodel=kernel", "-mno-mmx", "-mno-sse", "-mno-sse2", "-mno-avx"]
                [..],
        )),
        "aarch64" => Some(("aarch64-unknown-none-elf", &["-mgeneral-regs-only"][..])),
        // The Rust target's own ISA and ABI (RV64GC, lp64d): the FP save and
        // restore assemble only with D, and the kernel may sit anywhere.
        "riscv64" => Some((
            "riscv64-unknown-none-elf",
            &["-march=rv64gc", "-mabi=lp64d", "-mcmodel=medany"][..],
        )),
        _ => None,
    }
}
