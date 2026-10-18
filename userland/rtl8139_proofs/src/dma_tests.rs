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

//! The two DMA grants: the part's buffer registers are 32 bits wide, so both
//! are asked for below 4 GiB and each is held to ending there, not only to
//! starting there; a grant that does not is refused, and the device goes
//! back with nothing freed while the part could reach it.

use nonos_libc::{dma_flags, given_back, line_up, DmaMapOut, PioGrantOut, MK_DMA_MAP_DMA32};

use crate::part::{part, DEVICE, PIO_GRANT};
use crate::setup::dma::map_all;

const PIO: PioGrantOut =
    PioGrantOut { port_base: 0xC000, port_count: 256, _pad: 0, grant_id: PIO_GRANT };
const RX_LEN: u64 = 0xB000;

fn grant(device_addr: u64, length: u64, grant_id: u64) -> DmaMapOut {
    DmaMapOut { user_va: 0x10_0000, device_addr, length, grant_id }
}

#[test]
fn both_grants_are_asked_for_below_4_gib_and_taken_when_they_end_there() {
    let _part = part(false);
    line_up(&[grant((1 << 32) - RX_LEN, RX_LEN, 4), grant(0x2000, 0x2000, 5)]);
    let (rx, tx) = map_all(DEVICE, 1, &PIO).expect("both grants fit");
    assert_eq!((rx.grant_id, tx.grant_id), (4, 5));
    assert!(dma_flags().iter().all(|f| f & MK_DMA_MAP_DMA32 != 0), "{:?}", dma_flags());
}

#[test]
fn a_grant_that_starts_below_4_gib_and_ends_above_it_is_refused() {
    for (at, len) in [((1 << 32) - 0x1000, RX_LEN), (1 << 32, RX_LEN), (u64::MAX - 0xFFF, RX_LEN)] {
        let _part = part(false);
        line_up(&[grant(at, len, 4), grant(0x2000, 0x2000, 5)]);
        let before = given_back().len();
        assert_eq!(map_all(DEVICE, 1, &PIO).err(), Some("rtl8139 requires 32-bit dma"));
        assert_eq!(given_back()[before..], [("device_release", DEVICE)], "at {at:#x}");
    }
}

#[test]
fn a_refused_map_or_a_short_grant_gives_the_device_back() {
    let short = [grant(0x1000, RX_LEN, 4), grant(0x2000, 0x1000, 5)];
    for (grants, err) in
        [(&short[..1], "tx dma failed"), (&short[..], "rtl8139 dma grant shorter than asked")]
    {
        let _part = part(false);
        line_up(grants);
        let before = given_back().len();
        assert_eq!(map_all(DEVICE, 1, &PIO).err(), Some(err));
        assert_eq!(given_back()[before..], [("device_release", DEVICE)]);
    }
}
