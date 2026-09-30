/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Finding the Wi-Fi driver this machine runs.
//!
//! Init starts a Wi-Fi driver only when its device is present, and each
//! registers the handle its Capsule.mk names, so the first registered name is
//! the radio. The RTL8821CE is asked first: on a machine with both cards it is
//! the one that can join a network.

use core::ptr;

use nonos_libc::mk_service_lookup;

use super::call::call;

/// Each driver's registered service and the name the panels show for it.
const SERVICES: &[(&[u8], &str)] =
    &[(b"driver.rtl8821ce0", "Realtek RTL8821CE"), (b"driver.iwlwifi0", "Intel Wi-Fi")];

/// A registered Wi-Fi driver service.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Driver {
    port: u32,
    label: &'static str,
}

/// The Wi-Fi driver service that is registered, if any.
pub fn find() -> Option<Driver> {
    SERVICES.iter().find_map(|(name, label)| {
        let mut port: u32 = 0;
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, ptr::null_mut());
        (rc == 0 && port != 0).then_some(Driver { port, label })
    })
}

impl Driver {
    /// The adapter name for display.
    pub fn label(&self) -> &'static str {
        self.label
    }

    /// Send one control request and wait for the reply, for callers that read
    /// driver-specific fields (the RTL8821CE's data-path counters). Returns the
    /// reply length when at least a header came back.
    pub fn request(&self, op: u16, body: &[u8], resp: &mut [u8], timeout_ms: u64) -> Option<usize> {
        call(self.port, op, body, resp, timeout_ms)
    }
}
