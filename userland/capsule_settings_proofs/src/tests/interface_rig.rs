// NONOS Operating System (AGPL-3.0-or-later)

//! A wireless PCI function as the broker lists it, for the discovery proofs.

use crate::wifi_class::{DeviceView, BUS_KIND_PCI, PCI_CLASS_NETWORK, PCI_SUBCLASS_WIRELESS};

pub(crate) fn wifi(vendor: u16, device: u16) -> DeviceView {
    DeviceView {
        bus_kind: BUS_KIND_PCI,
        pci_class: PCI_CLASS_NETWORK,
        pci_subclass: PCI_SUBCLASS_WIRELESS,
        vendor,
        device,
    }
}
