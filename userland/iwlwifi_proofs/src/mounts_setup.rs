// NONOS Operating System (AGPL-3.0-or-later)
//! The driver sources the setup proofs run: the chip wake, the grants of an
//! attempt, the registers, and the adapter decision.

#[path = "../../capsule_driver_iwlwifi/src/constants/mod.rs"]
pub mod constants;
// The chip wake and the release of an attempt's grants when it fails, run
// against a register window in host memory.
#[path = "../../capsule_driver_iwlwifi/src/setup/claim.rs"]
pub mod claim;
#[path = "../../capsule_driver_iwlwifi/src/setup/command.rs"]
pub mod command;
#[path = "../../capsule_driver_iwlwifi/src/setup/grants.rs"]
pub mod grants;
#[path = "../../capsule_driver_iwlwifi/src/driver/mod.rs"]
pub mod driver;
#[path = "../../capsule_driver_iwlwifi/src/setup/give_back.rs"]
pub mod give_back;
#[path = "../../capsule_driver_iwlwifi/src/init.rs"]
pub mod init;
#[path = "../../capsule_driver_iwlwifi/src/mmio.rs"]
pub mod mmio;
#[path = "../../capsule_driver_iwlwifi/src/regs.rs"]
pub mod regs;
// The adapter decision discovery makes from a broker record, and what a
// refused interrupt bind means for the attempt: pure, so proven exactly.
pub mod firmware;
#[path = "../../capsule_driver_iwlwifi/src/setup/irq_plan.rs"]
pub mod irq_plan;
#[path = "../../capsule_driver_iwlwifi/src/pci_match.rs"]
pub mod pci_match;
