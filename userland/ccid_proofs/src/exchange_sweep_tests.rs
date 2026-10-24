// NONOS Operating System (AGPL-3.0-or-later)
//! One command's exchange over seeded reader streams, sound and then told
//! one lie: exchange() ends as the rule in exchange_sweep/model.rs says,
//! with the same waits begun afresh and pipes cleared, and as the lie
//! must leave it.

use nonos_usbnet::mock::Call;

use crate::ccid::exchange;
use crate::exchange_sweep::build::{stream, Stream};
use crate::exchange_sweep::bus::Lying;
use crate::exchange_sweep::lie::{tell, LIES};
use crate::exchange_sweep::model::{rule, Seen};
use crate::exchange_sweep::rng::{rounds, Rng};
use crate::exchange_tests::reader;

/// Time enough to read every transfer once, and one look more.
fn driver(s: &Stream) -> (Seen, Seen) {
    let looks = s.transfers.len() + 1;
    let mut bus = Lying { bus: reader(s.transfers.clone()), took: s.took, past: s.past };
    bus.bus.0.borrow_mut().bulk_out_fails = s.out_fails;
    let (mut fresh, mut seen) = (0, 0);
    let wait = |f: bool| {
        (fresh, seen) = (fresh + f as u32, seen + 1);
        seen <= looks
    };
    let result = exchange(&mut bus, &s.cmd, s.max, wait);
    let calls = bus.bus.0.borrow().calls.clone();
    let resets =
        calls.iter().filter_map(|c| if let Call::ResetBulk(i) = c { Some(*i) } else { None });
    let got = Seen { result, fresh, resets: resets.collect() };
    (got, rule(s, looks))
}

#[test]
fn every_sound_exchange_ends_in_our_answer() {
    for seed in 1..rounds(5_000) {
        let s = stream(&mut Rng::new(seed));
        let (got, model) = driver(&s);
        assert_eq!(got, model, "seed {seed}: the rule");
        assert_eq!(got.result, Ok(s.answer.clone()), "seed {seed}");
        assert!(got.resets.is_empty(), "seed {seed}");
    }
}

#[test]
fn after_any_lie_the_exchange_ends_as_the_lie_leaves_it() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(24_000) {
        let (lie, s, want) = tell(&mut Rng::new(seed));
        let (got, model) = driver(&s);
        assert_eq!(got, model, "seed {seed}: {lie:?}: the rule");
        assert_eq!(got.result, want.result, "seed {seed}: {lie:?}");
        assert_eq!(got.resets, want.resets, "seed {seed}: {lie:?}: pipes cleared");
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 1_500, "{lie:?} was told only {n} times");
    }
}
