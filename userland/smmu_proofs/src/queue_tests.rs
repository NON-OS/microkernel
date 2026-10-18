// NONOS Operating System (AGPL-3.0-or-later)
//! PROD and CONS with their wrap bit, over every position of small queues.

use crate::smmu::encode::queue::{advance, consumed, index, is_empty, is_full};

#[test]
fn filling_and_draining_a_queue() {
    for log2 in 1..=5u8 {
        let size = 1u32 << log2;
        let (mut prod, mut cons) = (0, 0);
        assert!(is_empty(prod, cons, log2));
        for n in 0..size {
            assert!(!is_full(prod, cons, log2), "log2 {log2} after {n}");
            prod = advance(prod, log2);
            assert!(!is_empty(prod, cons, log2));
        }
        assert!(is_full(prod, cons, log2));
        for _ in 0..size {
            cons = advance(cons, log2);
        }
        assert!(is_empty(prod, cons, log2));
        assert_eq!(index(prod, log2), 0);
        assert_eq!(prod, size, "one lap flips only the wrap bit");
        assert_eq!(advance(advance(prod, log2), log2) >> (log2 + 1), 0);
    }
}

#[test]
fn consumed_holds_exactly_once_cons_reaches_the_target() {
    for log2 in 1..=4u8 {
        let span = 2u32 << log2;
        for start in 0..span {
            // Put `ahead` commands in from `start`; CONS walks up to them.
            for ahead in 0..=(1u32 << log2) {
                let target = (start + ahead) % span;
                let mut cons = start;
                for step in 0..ahead {
                    assert!(!consumed(cons, target, log2), "{log2} {start} {ahead} {step}");
                    cons = advance(cons, log2);
                }
                assert!(consumed(cons, target, log2));
                // The ERR field above the counter does not confuse it.
                assert!(consumed(cons | 0x7f << 24, target, log2));
            }
        }
    }
}
