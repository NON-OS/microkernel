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

//! What the opener refuses: any changed bit, another LBA, another key.
//! A refused sector writes nothing where its plaintext would go.

use super::constants::{PLAIN_BLOCK_BYTES, SECTOR_BYTES};
use super::opener::open_sealed;
use super::seal_fixture::{fill, sealed};

#[test]
fn a_changed_sector_is_refused_and_writes_nothing() {
    let mut seed = 11;
    let (mut key, mut plain) = ([0u8; 32], [0u8; PLAIN_BLOCK_BYTES]);
    fill(&mut seed, &mut key);
    fill(&mut seed, &mut plain);
    let sector = sealed(&mut seed, &key, 42, &plain);
    for bit in 0..SECTOR_BYTES * 8 {
        let mut bad = sector;
        bad[bit / 8] ^= 1 << (bit % 8);
        let mut out = [0xa5; PLAIN_BLOCK_BYTES];
        assert!(!open_sealed(&key, 42, &bad, &mut out), "bit {bit}");
        assert!(out.iter().all(|&b| b == 0xa5), "bit {bit} wrote plaintext");
    }
}

#[test]
fn a_sector_is_refused_at_another_lba_or_under_another_key() {
    let mut seed = 13;
    let (mut key, mut plain) = ([0u8; 32], [0u8; PLAIN_BLOCK_BYTES]);
    fill(&mut seed, &mut key);
    fill(&mut seed, &mut plain);
    let sector = sealed(&mut seed, &key, 1000, &plain);
    let mut out = [0u8; PLAIN_BLOCK_BYTES];
    for lba in [0, 999, 1001, 1000 | 1 << 40, u64::MAX] {
        assert!(!open_sealed(&key, lba, &sector, &mut out), "lba {lba}");
    }
    let mut other = key;
    other[31] ^= 0x80;
    assert!(!open_sealed(&other, 1000, &sector, &mut out));
    assert!(out.iter().all(|&b| b == 0));
    assert!(open_sealed(&key, 1000, &sector, &mut out));
}
