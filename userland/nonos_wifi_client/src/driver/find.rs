/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Finding the Wi-Fi driver this machine runs.
//!
//! Init starts a Wi-Fi driver only when its device is present, and each
//! registers the handle its Capsule.mk names, so the first registered name is
//! the radio, in the order `services.rs` gives. The kernel holds those names
//! for the driver it spawned; the process that owns the name when it is found
//! must still own it, on the same port, when each request goes out.

use nonos_libc::mk_service_lookup;

use super::call::call;
use super::services::SERVICES;

/// A registered Wi-Fi driver service.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Driver {
    name: &'static [u8],
    port: u32,
    owner: u32,
    label: &'static str,
    joins: bool,
}

/// The Wi-Fi driver service that is registered, if any.
pub fn find() -> Option<Driver> {
    SERVICES.iter().find_map(|s| {
        let (port, owner) = holder(s.name)?;
        Some(Driver { name: s.name, port, owner, label: s.label, joins: s.joins })
    })
}

// The port and owning process of the service `name`, both nonzero.
fn holder(name: &[u8]) -> Option<(u32, u32)> {
    let (mut port, mut owner) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut owner);
    (rc == 0 && port != 0 && owner != 0).then_some((port, owner))
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
    /// reply length when the reply is this request's. Nothing is sent when
    /// the service's owner or port changed since it was found.
    pub fn request(&self, op: u16, body: &[u8], resp: &mut [u8], timeout_ms: u64) -> Option<usize> {
        if holder(self.name) != Some((self.port, self.owner)) {
            return None;
        }
        call(self.port, op, body, resp, timeout_ms)
    }
}
