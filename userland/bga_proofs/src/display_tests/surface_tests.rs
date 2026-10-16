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

//! The surface, made last: held to the kernel's 64 MiB bound on one, at it
//! and a page past it, and refused when the kernel will not map its
//! memory, with the firmware's state written back.

use nonos_libc::{lend, registered, untouched_since_unmap};

use super::left::{showing, state, text_mode, written_back};
use crate::adapter::{adapter, found, MODE_BYTES, REG_LEN};
use crate::constants::{DISPI_INDEX_VIDEO_MEMORY_64K, MAX_SURFACE_BYTES};
use crate::dispi::dispi_off;
use crate::error::BgaError;
use crate::setup::run;

#[test]
fn memory_the_kernel_will_not_map_refuses_the_surface_and_writes_the_state_back() {
    let a = adapter(REG_LEN, MODE_BYTES);
    text_mode(&a);
    let firmware = state(&a);
    lend(0, 0);
    assert_eq!(run(found(MODE_BYTES)).err(), Some(BgaError::SurfaceRefused));
    assert_eq!(registered(), [], "a surface was registered with no memory behind it");
    written_back(&a, 0, firmware);
    assert!(untouched_since_unmap(), "the register mapping was written after its unmap");
}

#[test]
fn a_surface_is_made_at_the_kernels_64_mib_bound_and_refused_a_page_past_it() {
    // 4096 x 4096 pixels are 64 MiB exactly; 1024 x 16385 are a page more.
    for (w, h) in [(4096u16, 4096u16), (1024, 16385)] {
        let bytes = w as usize * h as usize * 4;
        let a = adapter(REG_LEN, bytes);
        showing(&a, w, h);
        a.regs.present16(dispi_off(DISPI_INDEX_VIDEO_MEMORY_64K) as usize, u16::MAX);
        let r = run(found(bytes));
        if bytes as u64 == MAX_SURFACE_BYTES {
            let d = r.unwrap_or_else(|e| panic!("refused at the bound: {e:?}"));
            assert_eq!((d.width, d.height), (w, h));
        } else {
            assert_eq!(bytes as u64, MAX_SURFACE_BYTES + 4096);
            assert_eq!(r.err(), Some(BgaError::SurfaceRefused), "a page past the bound taken");
        }
    }
    let made: Vec<u64> = registered().iter().map(|s| s.byte_len).collect();
    assert_eq!(made, [MAX_SURFACE_BYTES], "not one surface, at the bound");
}
