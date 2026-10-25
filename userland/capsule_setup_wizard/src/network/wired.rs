//! Whether this machine has a wired network card.
//!
//! Init starts a wired driver only when its card is present, and net_core
//! binds any wired link that comes up and asks it for an address without
//! asking the person (capsule_net_core/src/setup.rs). So a registered wired
//! driver is what the step reports: it is used when a cable is plugged in.

use core::ptr;

use nonos_libc::mk_service_lookup;

// The kernel's table of the cards it starts, which net_core reads too.
#[path = "../../../../src/services/registry/nics.rs"]
mod nics;

use nics::{Link, NICS};

pub fn wired_present() -> bool {
    NICS.iter().filter(|(_, link)| *link == Link::Wired).any(|(name, _)| {
        let mut port: u32 = 0;
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, ptr::null_mut());
        rc == 0 && port != 0
    })
}
