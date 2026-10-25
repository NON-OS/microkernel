// NONOS Operating System (AGPL-3.0-or-later)

//! The candidate list is bounded: a flood of frames keeps the best, a join
//! request tries at most `MAX_TRIES` access points, an oversized frame is
//! not kept, and only a join the access point ended moves on to the next.

use nonos_wifi_core::bss::{moves_on, Candidates, FRAME_MAX, MAX_CANDIDATES, MAX_TRIES};
use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

use super::candidates::{psk, sae};
use super::sim::beacon;
use super::sim::AP;

pub(super) fn order(c: &mut Candidates) -> Vec<(Vec<u8>, u8)> {
    core::iter::from_fn(|| c.take_best()).map(|c| (c.frame().to_vec(), c.channel())).collect()
}

fn bssid(n: u8) -> [u8; 6] {
    [0x02, 0, 0, 0, 0x10, n]
}

#[test]
fn a_flood_keeps_the_best() {
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    let real = beacon(AP, &[&sae()]);
    c.hear(&real, 6, 40);
    for n in 0..4 * MAX_CANDIDATES as u8 {
        c.hear(&beacon(bssid(n), &[&psk()]), 1, 99);
    }
    assert_eq!(order(&mut c)[0], (real, 6), "WPA3 outranks every WPA2 flooded in");
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    for n in 0..MAX_CANDIDATES as u8 {
        c.hear(&beacon(bssid(n), &[&psk()]), 1, 10);
    }
    let better = beacon(AP, &[&psk()]);
    c.hear(&better, 1, 60);
    assert_eq!(order(&mut c)[0].0, better, "past the bound, a better one replaces the worst");
}

#[test]
fn at_most_max_tries_joins_and_no_oversized_frame() {
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    for n in 0..MAX_CANDIDATES as u8 {
        c.hear(&beacon(bssid(n), &[&psk()]), 1, n);
    }
    assert_eq!(order(&mut c).len(), MAX_TRIES);
    let mut big = beacon(AP, &[&psk()]);
    big.resize(FRAME_MAX + 1, 0xDD);
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    c.hear(&big, 1, 50);
    assert!(!c.found());
}

#[test]
fn only_a_join_the_access_point_ended_moves_on() {
    for f in [
        MlmeFailure::AuthRejected(1),
        MlmeFailure::AssocRejected(17),
        MlmeFailure::Sae(SaeFailure::BadConfirm),
        MlmeFailure::Left(15),
        MlmeFailure::Handshake(Failure::IeMismatch),
    ] {
        assert!(moves_on(Some(f)), "{f:?}");
    }
    assert!(moves_on(None), "the access point went quiet");
    for f in [MlmeFailure::BadPassphrase, MlmeFailure::SaeStart, MlmeFailure::OpenNetwork] {
        assert!(!moves_on(Some(f)), "{f:?}");
    }
}
