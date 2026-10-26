// NONOS Operating System (AGPL-3.0-or-later)

//! What the host enabled on the card's PCIe link, read once at bring-up from
//! its configuration space; nothing when a read is refused.

use nonos_libc::mk_pci_config_read;

use super::Mapped;
use crate::pcie::{link_control, Aspm};

pub fn host_aspm(m: &Mapped) -> Aspm {
    let read = |offset: u32, width: u32| {
        u32::try_from(mk_pci_config_read(m.device_id, m.claim_epoch, offset, width)).ok()
    };
    link_control(read).map(Aspm::from_link_control).unwrap_or_default()
}
