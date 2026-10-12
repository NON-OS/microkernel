// NONOS Operating System (AGPL-3.0-or-later)
//! The station address the search would draw for a run, fixed so the
//! driver proofs can say exactly what each device is given.

use crate::station::Station;

/// Six bytes as CryptoRandom might give them.
pub const RANDOM: [u8; 6] = [0x5b, 0x1d, 0x8e, 0x30, 0xc4, 0x27];

/// The station address made from them: 5a:1d:8e:30:c4:27.
pub fn drawn() -> Station {
    Station::from_random(RANDOM)
}
