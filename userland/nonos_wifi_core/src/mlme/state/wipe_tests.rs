// NONOS Operating System (AGPL-3.0-or-later)
//! The machine's secrets, and a join's randomness, read zero after the drop,
//! through raw pointers into storage the test keeps.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use super::fields::Mlme;
use super::request::{Entropy, JoinRequest, SAE_ENTROPY};
use crate::rsn::JoinPolicy;

fn entropy() -> Entropy {
    Entropy { snonce: [0x52; 32], sae: [0x5A; SAE_ENTROPY] }
}

#[test]
fn the_machines_secrets_read_zero_after_it_is_dropped() {
    let (ssid, passphrase) = (&b"home"[..], &b"correct horse"[..]);
    let req = JoinRequest {
        our_mac: [2, 0, 0, 0, 0, 1],
        ssid,
        passphrase,
        policy: JoinPolicy::ANY,
        entropy: entropy(),
    };
    let mut slot = MaybeUninit::new(Mlme::join(&req));
    let m = slot.as_mut_ptr();
    // SAFETY: `m` points at the Mlme `slot` holds; `slot` outlives every read
    // and never drops it again, so after `drop_in_place` its arrays still
    // hold the bytes the drop left, and only those arrays are read.
    unsafe {
        (*m).pmk = [0x33; 32];
        assert_eq!(read_volatile(addr_of!((*m).secret))[..13], *b"correct horse");
        drop_in_place(m);
        assert_eq!(read_volatile(addr_of!((*m).secret)), [0; 64], "the passphrase");
        assert_eq!(read_volatile(addr_of!((*m).pmk)), [0; 32], "the PMK");
        assert_eq!(read_volatile(addr_of!((*m).snonce)), [0; 32], "the SNonce");
        assert_eq!(read_volatile(addr_of!((*m).sae_entropy)), [0; SAE_ENTROPY], "SAE's bytes");
    }
}

#[test]
fn a_joins_randomness_reads_zero_after_it_is_dropped() {
    let mut slot = MaybeUninit::new(entropy());
    let e = slot.as_mut_ptr();
    // SAFETY: as above, for the Entropy `slot` holds.
    unsafe {
        drop_in_place(e);
        assert_eq!(read_volatile(addr_of!((*e).snonce)), [0; 32]);
        assert_eq!(read_volatile(addr_of!((*e).sae)), [0; SAE_ENTROPY]);
    }
}
