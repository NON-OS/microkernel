//! Whether this machine has a wired network card.
//!
//! Init starts a wired driver only when its card is present, and net_core
//! binds any wired link that comes up and asks it for an address without
//! asking the person (capsule_net_core/src/setup.rs). So a registered wired
//! driver is what the step reports: it is used when a cable is plugged in.

use core::ptr;

use nonos_libc::mk_service_lookup;

/// The wired drivers' registered services, as net_core lists them.
const WIRED: &[&[u8]] =
    &[b"driver.virtio_net0", b"driver.e1000_0", b"driver.rtl8169_0", b"driver.rtl8139_0"];

pub fn wired_present() -> bool {
    WIRED.iter().any(|name| {
        let mut port: u32 = 0;
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, ptr::null_mut());
        rc == 0 && port != 0
    })
}
