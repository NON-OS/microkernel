// NONOS Operating System (AGPL-3.0-or-later)

//! The station's first address, drawn at random (the efuse's address is
//! never read), and the environment the shared station runs in: the
//! uptime clock and the kernel's random source.

use nonos_mac::{apply, MAC_LEN};
use nonos_wifi_core::softmac::Env;

/// A random, locally administered unicast address, or `None` without
/// entropy.
pub fn draw() -> Option<[u8; MAC_LEN]> {
    let mut mac = [0u8; MAC_LEN];
    if !fill(&mut mac) {
        return None;
    }
    apply(&mut mac);
    Some(mac)
}

fn fill(out: &mut [u8]) -> bool {
    let rc = nonos_libc::crypto_random(out.as_mut_ptr(), out.len());
    rc >= 0 && rc as usize == out.len()
}

pub struct CapsuleEnv;

impl Env for CapsuleEnv {
    fn now_ms(&self) -> u64 {
        crate::clock::now_ms()
    }
    fn random(&mut self, out: &mut [u8]) -> bool {
        fill(out)
    }
}
