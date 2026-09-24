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

//! The counter behaviour a relay keystream depends on.

use super::aes_tests::block;
use nonos_aes::{Aes128, Ctr128Be};

const CELL: usize = 509;

/// The keystream must be the cipher applied to successive counter blocks from
#[test]
fn keystream_is_the_counter_encrypted() {
    let key = block("2b7e151628aed2a6abf7158809cf4f3c");
    let mut stream = vec![0u8; 48];
    Ctr128Be::new(&key).apply(&mut stream);
    for counter in 0u8..3 {
        let mut expected = [0u8; 16];
        expected[15] = counter;
        Aes128::new(&key).encrypt_block(&mut expected);
        let at = counter as usize * 16;
        assert_eq!(&stream[at..at + 16], &expected[..], "block {counter}");
    }
}

/*
 * A relay encrypts 509 byte payloads, so every cell after the first starts part
 * way through a keystream block. A counter that restarted per call, or that
 * rounded up to a block boundary, would encrypt the first cell correctly and
 * corrupt every one after it.
 */
#[test]
fn split_writes_are_one_continuous_keystream() {
    let key = block("000102030405060708090a0b0c0d0e0f");
    let mut once = vec![0u8; CELL * 3];
    Ctr128Be::new(&key).apply(&mut once);
    let mut split = vec![0u8; CELL * 3];
    let mut cipher = Ctr128Be::new(&key);
    for chunk in split.chunks_mut(CELL) {
        cipher.apply(chunk);
    }
    assert_eq!(once, split);
}
