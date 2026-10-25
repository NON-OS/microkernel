// NONOS Operating System (AGPL-3.0-or-later)
//! Runs of notifications, one of them told a lie, with the client's reads
//! between: serial_state and State read what the rule in
//! notice_sweep/model.rs reads, and the lied notice changes nothing.

use crate::acm::{serial_state, State};
use crate::notice_sweep::build::{lied, sound, LIES};
use crate::notice_sweep::model::{bits, Line};
use crate::notice_sweep::rng::{rounds, Rng};

#[test]
fn a_client_reads_the_lines_and_every_event_and_no_lie() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(20_000) {
        let r = &mut Rng::new(seed);
        let (comm, lie) = (r.range(0, 7) as u8, LIES[r.range(0, LIES.len() - 1)]);
        let (mut state, mut rule, mut want) = (State::default(), Line::default(), Line::default());
        let n = r.range(1, 8);
        let j = r.range(0, n - 1);
        for i in 0..n {
            let at = format!("seed {seed}: {lie:?}, notice {i}");
            let notice = if i == j {
                lied(r, comm, lie)
            } else {
                let (n, b) = sound(r, comm);
                want.seen(b);
                n
            };
            if i == j && lie == crate::notice_sweep::build::Lie::Sound {
                want.seen(bits(&notice, comm).unwrap());
            }
            let got = serial_state(&notice, comm);
            assert_eq!(got, bits(&notice, comm), "{at}: the rule");
            if let Some(b) = got {
                state.seen(b);
            }
            if let Some(b) = bits(&notice, comm) {
                rule.seen(b);
            }
            if r.range(0, 2) == 0 || i == n - 1 {
                let s = state.take();
                assert_eq!(s, rule.read(), "{at}: the rule's read");
                assert_eq!(s, want.read(), "{at}: the read");
            }
        }
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 2_500, "{lie:?} was told only {n} times");
    }
}
