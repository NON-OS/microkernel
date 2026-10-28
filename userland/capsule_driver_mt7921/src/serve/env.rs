/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The station's surroundings: the uptime clock and the kernel's
//! randomness (`CryptoRandom`), from which every SNonce, SAE secret and
//! per-network address is drawn.

use nonos_wifi_core::softmac::Env;

pub struct Kernel;

impl Env for Kernel {
    fn now_ms(&self) -> u64 {
        crate::clock::now_ms()
    }

    fn random(&mut self, out: &mut [u8]) -> bool {
        nonos_libc::crypto_random(out.as_mut_ptr(), out.len()) == out.len() as i64
    }
}

/// Which process owns net.core now, asked on each link request so a
/// restarted net.core is followed and none admits nobody.
pub fn net_core_pid() -> Option<u32> {
    const NET_CORE: &[u8] = b"net.core";
    let mut pid = 0u32;
    let found = nonos_libc::mk_service_lookup(
        NET_CORE.as_ptr(),
        NET_CORE.len(),
        core::ptr::null_mut(),
        &mut pid,
    );
    (found >= 0 && pid != 0).then_some(pid)
}
