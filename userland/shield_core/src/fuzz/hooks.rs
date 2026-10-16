// NONOS Operating System (AGPL-3.0-or-later)
//! Rounds through the readers that already keep a fuzzing hook: a swap's market, the lander's
//! replies and the rewards link state, fed the same hostile replies and words of every length
//! about one ABI word.

use super::reply::hostile;
use super::rng::Rng;
use crate::evm::Viewed;

/// The statuses a lander answers with, honest and not.
const STATUSES: [u16; 7] = [0, 200, 201, 400, 404, 429, 500];
/// View replies at and about the word bounds.
const LENS: [usize; 8] = [0, 1, 31, 32, 33, 64, 96, 128];

/// Run `rounds` rounds from `seed`; returns the number run.
pub fn hook_rounds(seed: u64, rounds: usize) -> usize {
    let mut rng = Rng::new(seed);
    for _ in 0..rounds {
        let text = hostile(&mut rng);
        crate::evm::swap::fuzz_hook::market(&text);
        crate::net::relay::fuzz_hook::replies(STATUSES[rng.below(STATUSES.len())], &text);
        let count = rng.below(6);
        let found: Vec<Viewed> = (0..count)
            .map(|_| {
                let len = LENS[rng.below(LENS.len())];
                let data = rng.bytes(len);
                if rng.below(5) == 0 {
                    Viewed::Reverted(data)
                } else {
                    Viewed::Data(data)
                }
            })
            .collect();
        let _ = crate::wallet::rewards::link_state(&found);
    }
    rounds
}
