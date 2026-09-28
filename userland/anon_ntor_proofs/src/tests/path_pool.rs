// NONOS Operating System (AGPL-3.0-or-later)
/* A relay pool and flat weights for the path tests. */

use crate::path::{Flags, Relay, Weights};

pub fn relay(address: [u8; 4], id: u8) -> Relay {
    Relay {
        address,
        or_port: 9001,
        rsa_identity: [id; 20],
        ed25519_identity: [id; 32],
        ntor_onion_key: [id; 32],
        flags: Flags {
            running: true,
            valid: true,
            fast: true,
            stable: true,
            guard: true,
            exit: true,
            authority: false,
        },
        weight: 1000,
        exits_web: true,
    }
}

pub fn flat() -> Weights {
    Weights {
        wgg: 10_000,
        wgd: 10_000,
        wmg: 10_000,
        wmd: 10_000,
        wme: 10_000,
        wmm: 10_000,
        wee: 10_000,
        wed: 10_000,
    }
}

pub fn pool() -> Vec<Relay> {
    (1..=12u8).map(|i| relay([10, i, 0, 1], i)).collect()
}
