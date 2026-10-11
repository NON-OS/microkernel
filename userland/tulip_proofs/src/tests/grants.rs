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

//! DMA grants: always below 4 GiB, narrowed by direction where the broker
//! knows the bits, and asked again without them where it does not.

use crate::setup::grant::{take, FROM_DEVICE, TO_DEVICE};
use nonos_libc::{dma_flags_asked, given_back, script_dma, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

#[test]
fn a_broker_that_knows_direction_is_asked_once() {
    script_dma(vec![Ok(0x0020_0000)]);
    let g = take(1, 1, 0x2000, 0, TO_DEVICE).expect("grant");
    assert_eq!(g.device_addr, 0x0020_0000);
    assert_eq!(dma_flags_asked(), vec![MK_DMA_MAP_DMA32 | TO_DEVICE]);
}

#[test]
fn a_broker_without_direction_bits_is_asked_again_without_them() {
    script_dma(vec![Err(-95), Ok(0x0020_0000)]);
    assert!(take(1, 1, 0x2000, 0, FROM_DEVICE).is_some());
    assert_eq!(dma_flags_asked(), vec![MK_DMA_MAP_DMA32 | FROM_DEVICE, MK_DMA_MAP_DMA32]);
}

#[test]
fn any_other_refusal_is_final() {
    script_dma(vec![Err(-12), Ok(0x0020_0000)]);
    assert!(take(1, 1, 0x1000, MK_DMA_MAP_COHERENT, 0).is_none());
    assert_eq!(dma_flags_asked(), vec![MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT]);
}

// The part would truncate the address and write somewhere else entirely.
#[test]
fn a_grant_above_4_gib_is_given_back_and_refused() {
    script_dma(vec![Ok(0xFFFF_F000)]);
    assert!(take(1, 1, 0x2000, 0, 0).is_none());
    assert_eq!(given_back(), vec![("dma_unmap", 1)]);
}
