// NONOS Operating System (AGPL-3.0-or-later)

//! The join's judge and the candidate list fed mutated beacons, fifty
//! thousand rounds. Nothing may panic; a frame judged joinable names the
//! network and carries a whole, parseable RSN element from inside itself;
//! every candidate taken is one the judge still accepts.

use nonos_wifi_core::bss::{Candidates, FRAME_MAX, MAX_TRIES};
use nonos_wifi_core::mlme::judge;
use nonos_wifi_core::rsn::{parse_rsne, JoinPolicy};

use super::candidates::{psk, sae};
use super::sim::beacon;
use super::sim::{AP, OTHER, SSID};
use crate::sweep::fuzz::Rng;

const ROUNDS: u32 = 50_000;

#[test]
fn judged_frames_and_candidates_hold_their_rule() {
    let seeds = [
        beacon(AP, &[&sae(), &[244, 1, 0x20], &[1, 2, 0x82, 0x84]]),
        beacon(OTHER, &[&psk(), &[50, 1, 0x0C]]),
    ];
    let mut r = Rng::seeded(0x0B55);
    for policy in [JoinPolicy::ANY, JoinPolicy::WPA3_ONLY] {
        let mut found = Candidates::new(SSID, policy);
        let mut joinable = 0u32;
        for round in 0..ROUNDS {
            let f = r.mutate(&seeds[round as usize % 2]);
            if let Some(Ok(j)) = judge(&f, SSID, policy) {
                joinable += 1;
                let (o, i) = (f.as_ptr() as usize, j.rsne.as_ptr() as usize);
                assert!(i >= o && i + j.rsne.len() <= o + f.len(), "round {round}");
                assert_eq!(j.rsne.len(), usize::from(j.rsne[1]) + 2, "round {round}");
                assert!(j.rsne[0] == 48 && parse_rsne(&j.rsne[2..]).is_some(), "round {round}");
                assert_eq!(j.info.ssid, SSID, "round {round}");
            }
            found.hear(&f, 1 + r.below(13) as u8, r.byte());
        }
        assert!(joinable > ROUNDS / 10, "the mutations reach the judge: {joinable}");
        let mut taken = 0;
        while let Some(c) = found.take_best() {
            taken += 1;
            assert!(c.frame().len() <= FRAME_MAX);
            assert!(matches!(judge(c.frame(), SSID, policy), Some(Ok(_))));
        }
        assert!((1..=MAX_TRIES).contains(&taken));
    }
}
