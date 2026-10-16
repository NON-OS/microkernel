// NONOS Operating System (AGPL-3.0-or-later)
//! A model of one AHCI port and the SATA drive on its link, behind the host
//! MMIO accessor, so the proofs run the driver's command path end to end:
//! issue, wait, recovery and the next command, against a drive that fails,
//! stalls, keeps working after the driver gave up, or moves short.

mod bare_port;
mod block;
mod drive;
mod fetch;
mod hba;
mod hba_addr;
mod hba_bus;
mod hba_cmd;
mod hba_end;
mod hba_global;
mod hba_order;
mod hba_ports;
mod hba_rig;
mod hba_run;
mod identify;
mod iommu;
mod medium;
mod plug;
mod quirks;
mod rig;
mod rig_io;
mod script;
mod state;
mod tests;

pub use bare_port::bare_port;
pub use hba_rig::HbaRig;
pub use identify::{CAPACITY, MODEL};
pub use medium::pattern;
pub use plug::{read, write};
pub use quirks::CrStuck;
pub use rig::Rig;
pub use script::{Outcome, Step};
