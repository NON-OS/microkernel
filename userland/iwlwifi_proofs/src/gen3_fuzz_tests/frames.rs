// NONOS Operating System (AGPL-3.0-or-later)
//! Frames off the air as the firmware hands them over: a management frame
//! for the scan and the join, and a data MPDU for the link.

use crate::fuzz::Rng;
use crate::gen3::rx_data::parse as rx_data;
use crate::gen3::rx_frame::{parse as rx_frame, DESC_LEN};
use crate::join_rig::{beacon, eapol_from_ap, rx, RSNE_PSK};

const ROUNDS: u32 = 40_000;

#[test]
fn a_received_frame_is_never_longer_than_what_carried_it() {
    let seeds = [
        rx(&beacon(&[&RSNE_PSK]), 0).payload,
        rx(&eapol_from_ap(&[1, 3, 0, 4, 2, 0, 0, 0], 7), 0).payload,
    ];
    let mut r = Rng::seeded(0xF4A);
    let mut taken = 0u32;
    for round in 0..ROUNDS {
        let p = if round % 4 == 0 { r.noise(400) } else { r.mutate(&seeds[round as usize % 2]) };
        if let Some(f) = rx_frame(&p) {
            assert!(f.frame.len() + DESC_LEN <= p.len(), "round {round}");
            taken += 1;
        }
        if let Some(m) = rx_data(&p) {
            assert!(m.frame.len() + DESC_LEN <= p.len(), "round {round}");
        }
    }
    assert!(taken > ROUNDS / 20, "the mutations reach a parsed frame: {taken}");
}
