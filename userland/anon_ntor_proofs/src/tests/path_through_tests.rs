// NONOS Operating System (AGPL-3.0-or-later)
//! A circuit's path starts at the guard the link is open to.

use crate::path::through;

use super::path_pool::{flat, pool, relay};

#[test]
fn the_first_hop_is_the_linked_guard_for_every_roll() {
    let relays = pool();
    let guard = relays[4].clone();
    for seed in 0..500u64 {
        let mut n = seed;
        let path = through(&guard, &relays, &flat(), || {
            n = n.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            Some(n)
        })
        .expect("a twelve relay pool gives a path");
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].rsa_identity, guard.rsa_identity);
        assert_ne!(path[1].rsa_identity, guard.rsa_identity);
        assert_ne!(path[2].rsa_identity, guard.rsa_identity);
        assert_ne!(path[1].rsa_identity, path[2].rsa_identity);
    }
}

#[test]
fn no_later_hop_shares_the_guards_16() {
    let mut relays = pool();
    // Same /16 as the guard: never a middle or an exit.
    relays.push(relay([10, 5, 9, 9], 99));
    let guard = relays[4].clone();
    for seed in 0..500u64 {
        let mut n = seed;
        let path = through(&guard, &relays, &flat(), || {
            n = n.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
            Some(n)
        })
        .unwrap();
        assert!(path[1..].iter().all(|r| r.rsa_identity != [99; 20]));
    }
}

#[test]
fn no_dice_no_path() {
    let relays = pool();
    assert!(through(&relays[0], &relays, &flat(), || None).is_none());
}
