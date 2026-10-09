/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Finding the Wi-Fi driver this machine runs.
//!
//! Init starts a Wi-Fi driver only when its device is present, and each
//! registers the handle its Capsule.mk names, so the first registered name is
//! the radio, in the order `services.rs` gives.

use core::ptr;

use nonos_libc::mk_service_lookup;

use super::call::call;
use super::services::SERVICES;

/// A registered Wi-Fi driver service.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Driver {
    port: u32,
    label: &'static str,
    joins: bool,
}

/// The Wi-Fi driver service that is registered, if any.
pub fn find() -> Option<Driver> {
    SERVICES.iter().find_map(|s| {
        let mut port: u32 = 0;
        let rc = mk_service_lookup(s.name.as_ptr(), s.name.len(), &mut port, ptr::null_mut());
        (rc == 0 && port != 0).then_some(Driver { port, label: s.label, joins: s.joins })
    })
}

impl Driver {
    /// The adapter name for display.
    pub fn label(&self) -> &'static str {
        self.label
    }

    /// Whether this driver runs a join. One that does not is never sent a
    /// passphrase.
    pub fn joins(&self) -> bool {
        self.joins
    }

    /// Send one control request and wait for the reply, for callers that read
    /// driver-specific fields (the RTL8821CE's data-path counters). Returns the
    /// reply length when at least a header came back.
    pub fn request(&self, op: u16, body: &[u8], resp: &mut [u8], timeout_ms: u64) -> Option<usize> {
        call(self.port, op, body, resp, timeout_ms)
    }
}
