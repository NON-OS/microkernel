// NONOS Operating System (AGPL-3.0-or-later)
// The driver's firmware family table, mounted where `pci_match` names it.
#[path = "../../../capsule_driver_iwlwifi/src/firmware/family.rs"]
pub mod family;
#[path = "../../../capsule_driver_iwlwifi/src/firmware/generation.rs"]
pub mod generation;
// The gen3 tree, where the server's join messages name it.
pub use crate::gen3;
