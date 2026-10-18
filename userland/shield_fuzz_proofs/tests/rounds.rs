// NONOS Operating System (AGPL-3.0-or-later)
//! Every round set, from several seeds. A round asserts its reader's promise, so a failure names
//! the seed that replays it.

use shield_fuzz_proofs::fuzz::{batch_rounds, hook_rounds, note_rounds, reply_rounds, word_rounds};

const SEEDS: [u64; 6] = [1, 2, 3, 0x5eed, 0xdead_beef, u64::MAX];

#[test]
fn replies_are_read_whole_or_refused() {
    for seed in SEEDS {
        assert_eq!(reply_rounds(seed, 300), 300, "seed {seed}");
    }
}

#[test]
fn a_batch_holds_its_slots() {
    for seed in SEEDS {
        assert_eq!(batch_rounds(seed, 200), 200, "seed {seed}");
    }
}

#[test]
fn a_note_opens_only_whole_and_untouched() {
    for seed in SEEDS {
        assert_eq!(note_rounds(seed, 40), 40, "seed {seed}");
    }
}

#[test]
fn an_abi_word_decodes_strictly() {
    for seed in SEEDS {
        assert_eq!(word_rounds(seed, 2_000), 2_000, "seed {seed}");
    }
}

#[test]
fn the_swap_lander_and_rewards_readers_take_any_reply() {
    for seed in SEEDS {
        assert_eq!(hook_rounds(seed, 200), 200, "seed {seed}");
    }
}
