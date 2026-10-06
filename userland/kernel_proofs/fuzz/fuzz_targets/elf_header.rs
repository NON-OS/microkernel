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

//! Any bytes as a capsule ELF: the kernel's header reader never panics, and a
//! program header table it accepts lies wholly inside the file, in entries of
//! the size the loader reads. The magic is checked later, by the loader.

#![no_main]

use kernel_proofs::elf::{parse_header, program_header_bounds};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(h) = parse_header(data) else {
        return;
    };
    let Ok((off, size, count)) = program_header_bounds(data, &h) else {
        return;
    };
    if count > 0 {
        assert!(size == 56, "a program header entry of {size} bytes");
        assert!(off + size * count <= data.len(), "the table runs past the file");
    }
});
