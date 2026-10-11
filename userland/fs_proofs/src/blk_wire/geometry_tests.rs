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

use super::geometry::Geometry;
use super::transfer::DATA_BYTES;

#[test]
fn an_emmc_geometry_is_the_sata_one() {
    // driver.sdhci0 answers 512-byte blocks and a 32 KiB transfer.
    assert_eq!(Geometry::wire(512, 32_768), Some(Geometry::SECTORS));
}

#[test]
fn larger_blocks_and_transfers_are_held_to_the_clients_buffer() {
    let g = Geometry::wire(4096, 1 << 20).expect("4K blocks");
    assert_eq!((g.lba_size(), g.max_bytes()), (4096, DATA_BYTES));
    let g = Geometry::wire(512, 4096 + 100).expect("ragged limit");
    assert_eq!(g.max_bytes(), 4096, "cut to whole blocks");
}

#[test]
fn a_block_size_or_limit_the_client_cannot_address_is_refused() {
    for (lba, max) in [(0, 32_768), (256, 32_768), (520, 32_768), (65_536, 1 << 20), (4096, 2048)] {
        assert_eq!(Geometry::wire(lba, max), None, "{lba} / {max}");
    }
}
