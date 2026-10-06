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

//! Up to a gigabyte, the span is the map libc always made. `old_map` is
//! `init_sized`'s mapping as it stood at 6ccadcc69, before the span, quoted
//! line for line. Both run against the same kernel, for every size a capsule
//! asks for up to the kernel's limit and every kind of answer, and must make
//! the same calls and give the same base or the same refusal.

use super::span::map;
use crate::mem::{calls, mk_mmap, reset};

const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_PRIVATE: i32 = 0x02;
const MAP_ANONYMOUS: i32 = 0x20;
const USERSPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

fn old_map(bytes: usize) -> Option<*mut u8> {
    let base = mk_mmap(
        core::ptr::null_mut(),
        bytes,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    let base_addr = base as u64;
    if base.is_null() || (base as i64) < 0 || base_addr > USERSPACE_MAX {
        return None;
    }
    Some(base)
}

/* The default, then every capsule's own size (vfs, browser, codec, viewer,
 * linux, terminal, installer, players), then the edges up to the limit. */
const CAPSULES: [usize; 7] =
    [16 << 20, 128 << 20, 96 << 20, 192 << 20, 320 << 20, 64 << 20, 32 << 20];
const EDGES: [usize; 7] = [0, 1, 4095, 4096, 4097, (1 << 30) - 4096, 1 << 30];

#[test]
fn up_to_a_gigabyte_the_span_is_the_old_map() {
    let above = (USERSPACE_MAX + 1) as isize;
    let answers = [None, Some((1, -12)), Some((1, -14)), Some((1, 0)), Some((1, above))];
    for base in [0x8000_0000, 0x7FFF_0000_0000] {
        for &bytes in CAPSULES.iter().chain(&EDGES) {
            for answer in answers {
                reset(base, answer, None);
                let old = (old_map(bytes).map(|p| p as usize), calls());
                reset(base, answer, None);
                let new = (map(bytes).map(|p| p as usize), calls());
                assert_eq!(old, new, "{bytes} bytes at {base:#x}, answer {answer:?}");
            }
        }
    }
}
