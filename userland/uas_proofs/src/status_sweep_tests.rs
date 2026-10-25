// NONOS Operating System (AGPL-3.0-or-later)
//! A command's status pipe over seeded exchanges, built sound and then told
//! one lie: the exchange in nonos_uas, driven as disk/uas.rs drives it,
//! comes to exactly what the rule in status_sweep/model.rs says and exactly
//! what the lie must leave.

use nonos_uas::{status_iu, DataDir, Exchange, ScsiCommand, Step};

use crate::status_sweep::build::exchange;
use crate::status_sweep::lie::{tell, LIES};
use crate::status_sweep::model::{outcome, Outcome};
use crate::status_sweep::rng::{rounds, Rng};

struct Cmd(DataDir, Option<(u8, Vec<u8>)>);

impl ScsiCommand for Cmd {
    fn cdb(&self) -> &[u8] {
        &[0x28, 0, 0, 0, 0, 0, 0, 0, 1, 0]
    }
    fn dir(&self) -> DataDir {
        self.0
    }
    fn length(&self) -> u32 {
        512
    }
    fn finished(&mut self, status: u8, sense: &[u8]) {
        self.1 = Some((status, sense.to_vec()));
    }
}

/// The loop disk/uas.rs runs, the pipe giving `ius` in turn.
fn driven(tag: u16, dir: DataDir, streams: bool, ius: &[Vec<u8>]) -> Outcome {
    let (mut x, mut cmd, mut next) = (Exchange::new(tag, dir, streams), Cmd(dir, None), ius.iter());
    let mut step = x.sent();
    loop {
        step = match step {
            Step::Status => {
                let Some(b) = next.next() else { return Outcome::Starved };
                let Some(iu) = status_iu(b) else { return Outcome::Unreadable };
                x.status(iu, &mut cmd)
            }
            Step::DataIn | Step::DataOut => x.moved(),
            Step::Done => break,
            Step::Failed(code) => return Outcome::Failed(code),
        };
    }
    cmd.1.map_or(Outcome::Starved, |(s, sense)| Outcome::Done(s, sense))
}

#[test]
fn every_sound_exchange_ends_with_its_status_and_sense() {
    for seed in 1..rounds(30_000) {
        let x = exchange(&mut Rng::new(seed));
        let want = Outcome::Done(x.status, x.sense.clone());
        assert_eq!(driven(x.tag, x.dir, x.streams, &x.ius), want, "seed {seed}");
        assert_eq!(outcome(x.tag, x.dir, x.streams, &x.ius), want, "seed {seed}: the rule");
    }
}

#[test]
fn after_any_lie_the_command_comes_to_what_the_rule_and_the_lie_say() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(60_000) {
        let (lie, x, want) = tell(&mut Rng::new(seed));
        let got = driven(x.tag, x.dir, x.streams, &x.ius);
        assert_eq!(got, outcome(x.tag, x.dir, x.streams, &x.ius), "seed {seed}: {lie:?}");
        assert_eq!(got, want, "seed {seed}: {lie:?}");
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 2_000, "{lie:?} was told only {n} times");
    }
}
