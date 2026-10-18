// NONOS Operating System (AGPL-3.0-or-later)
//! The CTAPHID assembler over seeded report streams, framed sound and then
//! told one lie: it hears exactly what the rule in frame_sweep/model.rs
//! hears, and exactly the honest messages the lie leaves, each whole.

use crate::ctaphid_parts::assemble::Assembler;
use crate::ctaphid_parts::heard::Heard;
use crate::frame_sweep::build::{stream, CID};
use crate::frame_sweep::lie::{tell, LIES};
use crate::frame_sweep::model::heard;
use crate::frame_sweep::rng::{rounds, Rng};

fn driver(reports: &[Vec<u8>]) -> Vec<Heard> {
    let mut a = Assembler::new(CID);
    reports.iter().map(|p| a.feed(p)).filter(|h| *h != Heard::Nothing).collect()
}

fn messages(heard: &[Heard]) -> Vec<(u8, Vec<u8>)> {
    heard
        .iter()
        .filter_map(|h| if let Heard::Message(c, d) = h { Some((*c, d.clone())) } else { None })
        .collect()
}

#[test]
fn every_sound_stream_is_heard_message_by_message() {
    for seed in 1..rounds(5_000) {
        let s = stream(&mut Rng::new(seed));
        let got = driver(&s.reports);
        let want: Vec<(u8, Vec<u8>)> = s.messages.iter().map(|m| (m.0, m.1.clone())).collect();
        assert_eq!(messages(&got), want, "seed {seed}");
        assert_eq!(got, heard(&s.reports, CID), "seed {seed}: the rule");
        assert!(!got.contains(&Heard::Broken), "seed {seed}");
    }
}

#[test]
fn after_any_lie_exactly_the_honest_messages_are_heard() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(30_000) {
        let (lie, s, kept) = tell(&mut Rng::new(seed));
        let got = driver(&s.reports);
        assert_eq!(got, heard(&s.reports, CID), "seed {seed}: {lie:?}");
        let want: Vec<(u8, Vec<u8>)> =
            kept.iter().map(|&m| (s.messages[m].0, s.messages[m].1.clone())).collect();
        assert_eq!(messages(&got), want, "seed {seed}: {lie:?}: the honest messages left");
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 1_500, "{lie:?} was told only {n} times");
    }
}
