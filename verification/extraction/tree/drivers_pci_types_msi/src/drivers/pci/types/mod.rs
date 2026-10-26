// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/drivers/pci/types/msi.rs"]
pub mod msi;

#[path = "../../../../../../../../src/drivers/pci/types/msi_message.rs"]
pub mod msi_message;

pub use msi::{MsiInfo, MsixInfo};
pub use msi_message::MsiMessage;
