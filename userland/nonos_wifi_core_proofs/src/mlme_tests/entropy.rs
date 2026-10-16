// NONOS Operating System (AGPL-3.0-or-later)
//! A join's randomness is drawn whole or not at all: a source that fails, on
//! either read, gives no `Entropy`, and `Mlme::join` takes nothing else, so
//! no join begins and no frame goes out on zeros.

use nonos_wifi_core::mlme::{Entropy, SAE_ENTROPY};

#[test]
fn a_source_that_fails_on_either_read_gives_no_randomness() {
    for fails_at in 0..2 {
        let mut reads = 0;
        let drawn = Entropy::draw(|b| {
            reads += 1;
            b.fill(0xAA);
            reads - 1 != fails_at
        });
        assert!(drawn.is_none(), "read {fails_at} failed");
        assert_eq!(reads, fails_at + 1, "nothing is read after a failure");
    }
}

#[test]
fn a_source_that_fills_both_reads_gives_them_unchanged() {
    let mut lens = Vec::new();
    let mut next = 0u8;
    let drawn = Entropy::draw(|b| {
        lens.push(b.len());
        b.iter_mut().for_each(|x| (*x, next) = (next, next.wrapping_add(1)));
        true
    })
    .expect("both reads filled");
    assert_eq!(lens, [32, SAE_ENTROPY], "the SNonce first, then SAE's own bytes");
    assert_eq!(drawn.snonce[31], 31);
    assert_eq!((drawn.sae[0], drawn.sae[SAE_ENTROPY - 1]), (32, (32 + SAE_ENTROPY - 1) as u8));
}
