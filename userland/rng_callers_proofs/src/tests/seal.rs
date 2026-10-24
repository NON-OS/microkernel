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

//! The sealed sector's nonce (fs/cryptoblock/seal.rs): drawn only from the
//! seeded RNG, and with none, no sector is sealed at all.

use crate::fs::cryptoblock::{open, seal, CryptoBlockError, PLAIN_BLOCK_BYTES};
use crate::stand_in::rng;

const KEY: [u8; 32] = [7; 32];

fn block() -> [u8; PLAIN_BLOCK_BYTES] {
    core::array::from_fn(|i| i as u8)
}

#[test]
fn seal_refuses_with_no_randomness_and_gives_no_sector() {
    rng::refuse();
    assert_eq!(seal(&KEY, 9, &block()), Err(CryptoBlockError::NoRandomness));
    assert_eq!(rng::draws(), 1);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn seal_refuses_a_wrong_length_before_drawing() {
    rng::fail_from(None);
    assert_eq!(seal(&KEY, 9, &[0; 3]), Err(CryptoBlockError::InvalidLength));
    assert_eq!(rng::draws(), 0);
}

#[test]
fn a_sealed_sector_carries_the_drawn_nonce_and_opens() {
    rng::fail_from(None);
    let sector = seal(&KEY, 9, &block()).expect("sealed");
    assert_eq!(rng::given(), [sector[..12].to_vec()]);
    assert_eq!(open(&KEY, 9, &sector), Ok(block()));
    assert_eq!(open(&KEY, 10, &sector), Err(CryptoBlockError::AuthenticationFailed));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn two_seals_of_one_block_draw_two_nonces() {
    rng::fail_from(None);
    let a = seal(&KEY, 9, &block()).expect("sealed");
    let b = seal(&KEY, 9, &block()).expect("sealed");
    assert_ne!(a[..12], b[..12]);
    assert_ne!(a[12..], b[12..]);
    assert_eq!(open(&KEY, 9, &a), open(&KEY, 9, &b));
}
