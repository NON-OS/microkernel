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

use super::memory_type::{choose, fold, KernelView, MemType, TypeRefusal, MMIO_MAP_WC};
use KernelView::{Mapped, Mixed, Unmapped};
use MemType::{Uncached as Uc, WriteCombining as Wc};

#[test]
fn registers_stay_uncached_and_a_frame_buffer_may_combine() {
    assert_eq!(choose(0, false, Unmapped), Ok(Uc));
    assert_eq!(choose(0, true, Unmapped), Ok(Uc));
    assert_eq!(choose(MMIO_MAP_WC, true, Unmapped), Ok(Wc));
    assert_eq!(choose(MMIO_MAP_WC, false, Unmapped), Err(TypeRefusal::NotPrefetchable));
}

#[test]
fn the_kernels_own_mapping_decides_the_type() {
    // The boot frame buffer: whatever a capsule asks, it gets the kernel's WC.
    assert_eq!(choose(0, true, Mapped(Wc)), Ok(Wc));
    assert_eq!(choose(MMIO_MAP_WC, true, Mapped(Wc)), Ok(Wc));
    // A range the kernel keeps uncached is never combined under it.
    assert_eq!(choose(MMIO_MAP_WC, true, Mapped(Uc)), Ok(Uc));
    assert_eq!(choose(0, false, Mapped(Uc)), Ok(Uc));
    for flags in [0, MMIO_MAP_WC] {
        assert_eq!(choose(flags, true, Mixed), Err(TypeRefusal::Mixed));
    }
}

#[test]
fn unknown_flags_are_refused_before_anything_else() {
    for bit in 1..32 {
        for view in [Unmapped, Mapped(Uc), Mapped(Wc), Mixed] {
            let flags = 1u32 << bit;
            assert_eq!(choose(flags, true, view), Err(TypeRefusal::UnknownFlags));
            assert_eq!(choose(flags | MMIO_MAP_WC, true, view), Err(TypeRefusal::UnknownFlags));
        }
    }
}

#[test]
fn a_capsule_never_gets_a_type_the_kernel_maps_differently() {
    for flags in [0, MMIO_MAP_WC] {
        for prefetchable in [false, true] {
            for kernel in [Uc, Wc] {
                if let Ok(t) = choose(flags, prefetchable, Mapped(kernel)) {
                    assert_eq!(t, kernel);
                }
            }
        }
    }
}

#[test]
fn folding_the_kernels_mappings() {
    assert_eq!(fold([]), Unmapped);
    assert_eq!(fold([Wc]), Mapped(Wc));
    assert_eq!(fold([Uc, Uc, Uc]), Mapped(Uc));
    assert_eq!(fold([Wc, Uc]), Mixed);
    assert_eq!(fold([Uc, Wc, Uc]), Mixed);
}
