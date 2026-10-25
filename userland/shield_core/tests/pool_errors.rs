//! Every revert the production pool and its AmountPolicy raise has its own sentence, each selector
//! derived here from its declaration in L1-Shield; FeeExceedsCap, which nothing raises, has none.

use nox_shield_core::net::pool_errors::explain;
use sha3::{Digest, Keccak256};

const RAISED: [&str; 15] = [
    "NotBeforeOffGrid(uint256)",
    "NotYet(uint256)",
    "TooManyIntents()",
    "NonUniformClearingPrice()",
    "WrongMsgValue()",
    "NonStandardTokenTransfer()",
    "SoundnessBelowFloor(uint256,uint256)",
    "PriceOutOfRange()",
    "FeeBelowProtocolPart(uint256,uint256)",
    "FeeNotOnLadder(uint256)",
    "GasPartWithoutSubmitter(uint256)",
    "NonStandardAmount()",
    "IsPaused(uint64)",
    "NoSchedule()",
    "NotRanged()",
];

fn selector(sig: &str) -> Vec<u8> {
    Keccak256::digest(sig.as_bytes())[..4].to_vec()
}

#[test]
fn each_raised_error_reads_as_its_own_sentence() {
    let mut seen = Vec::new();
    for sig in RAISED {
        let said = explain(&selector(sig));
        assert!(!said.contains("(0x"), "{sig} has no sentence");
        assert!(!seen.contains(&said), "{sig} shares a sentence");
        seen.push(said);
    }
    assert_eq!(selector("IsPaused(uint64)"), [0xd7, 0x5a, 0x7f, 0xb8]);
}

#[test]
fn a_cap_the_pool_does_not_have_is_not_explained() {
    assert_eq!(explain(&selector("FeeExceedsCap()")), "The pool refused the call (0xbc9c0f18).");
}
