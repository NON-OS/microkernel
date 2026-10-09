/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::indexing_slicing, clippy::unwrap_used, clippy::arithmetic_side_effects)]

use super::{address_limbs, pool_words, LIMBS};

/// withdraw-eth from the pinned 37-limb wallet vectors: 0.01 ETH out to 0x8ba1f109...dba72,
/// fee 0.00255 ETH to `address(1)`, not before 1,790,000,400.
const WITHDRAW_A: [u64; LIMBS] = [
    11465839100609169239,
    8902674681016596701,
    14023130704075225168,
    9782059357716360831,
    11465839100609169239,
    8902674681016596701,
    14023130704075225168,
    9782059357716360831,
    889919277466145136,
    8336274146572230245,
    5336986709291164636,
    1534744628385256744,
    18334764714270447212,
    1548061196981560856,
    8853329630701882500,
    5235092394456204384,
    5344353063958072554,
    15770223284851138842,
    8768739946316290274,
    2276447860313697381,
    11952601469125124443,
    5815868844311164110,
    7553101690732633827,
    9734593768947697372,
    10000000000000000,
    2550000000000000,
    0,
    1000000,
    60326411090546,
    140943955352257,
    265022384886834,
    35745,
    1,
    0,
    0,
    0,
    1790000400,
];

/// The pool's own expansion, `PublicWords.publicsOf` on a thirteen word intent.
fn expand(words: &[[u8; 32]; 13]) -> Vec<u64> {
    let limb =
        |w: &[u8; 32], from: usize| u64::from_be_bytes(w[from..from + 8].try_into().unwrap());
    let mut out = Vec::new();
    for (i, w) in words.iter().enumerate() {
        if (10..=11).contains(&i) {
            let mut wide = [0u8; 16];
            wide.copy_from_slice(&w[16..]);
            let low = u128::from_be_bytes(wide);
            let high = u128::from(u32::from_be_bytes(w[12..16].try_into().unwrap()));
            let n = |v: u128| u64::try_from(v & ((1 << 48) - 1)).unwrap();
            out.extend([
                n(low),
                n(low >> 48),
                n((low >> 96) | (high << 32)),
                u64::try_from(high >> 16).unwrap(),
            ]);
        } else if i <= 5 {
            out.extend([limb(w, 24), limb(w, 16), limb(w, 8), limb(w, 0)]);
        } else {
            assert!(w[..24].iter().all(|b| *b == 0));
            out.push(limb(w, 24));
        }
    }
    out
}

#[test]
fn the_words_expand_back_to_the_pinned_proofs_statement() {
    assert_eq!(expand(&pool_words(&WITHDRAW_A)), WITHDRAW_A.to_vec());
}

#[test]
fn address_limbs_match_the_pinned_withdrawal() {
    let recipient: [u8; 20] = [
        0x8b, 0xa1, 0xf1, 0x09, 0x55, 0x1b, 0xd4, 0x32, 0x80, 0x30, 0x12, 0x64, 0x5a, 0xc1, 0x36,
        0xdd, 0xd6, 0x4d, 0xba, 0x72,
    ];
    assert_eq!(
        address_limbs(&recipient),
        [WITHDRAW_A[28], WITHDRAW_A[29], WITHDRAW_A[30], WITHDRAW_A[31]]
    );
    let mut submitter = [0u8; 20];
    submitter[19] = 1;
    assert_eq!(
        address_limbs(&submitter),
        [WITHDRAW_A[32], WITHDRAW_A[33], WITHDRAW_A[34], WITHDRAW_A[35]]
    );
    assert_eq!(address_limbs(&[0xff; 20]), [(1 << 48) - 1, (1 << 48) - 1, (1 << 48) - 1, 0xffff]);
}
