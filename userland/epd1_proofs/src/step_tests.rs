// NONOS Operating System (AGPL-3.0-or-later)
//! Every order of the three events, up to eight long: the tables built, an
//! exposed CPU asking, a user root made. Whatever the order, no CPU ever
//! runs its vectors through the trampoline while some user root lacks it,
//! and the trampoline is used whenever an exposed CPU comes in time.

use crate::orders::{every, run, Event};

#[test]
fn no_cpu_switches_while_a_root_lacks_the_trampoline() {
    for len in 0..=8 {
        every(len, &mut |order| {
            let r = run(order);
            assert!(r.switched == 0 || r.without == 0, "{order:?}");
            assert!(r.switched > 0 || r.with == 0, "a root carries it for nothing: {order:?}");
            assert!(!r.early, "an exposed CPU in time was refused: {order:?}");
            if !order.iter().any(|e| matches!(e, Event::Built)) {
                assert_eq!(r.switched, 0, "switched before the tables: {order:?}");
            }
        });
    }
}
