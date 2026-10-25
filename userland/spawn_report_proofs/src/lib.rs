// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for init's last spawn line. The failure count, the report and
//! the spawn error are the kernel's own files; the console is a stand-in that
//! keeps what was written.

#![allow(dead_code)]

pub mod console;

#[path = "../../../src/userspace/init/capsule_boot/failures.rs"]
mod failures;
#[path = "../../../src/kernel_core/process_spawn/capsule_spawn/spawn_error.rs"]
mod spawn_error;
#[path = "../../../src/userspace/init/spawn_report.rs"]
mod spawn_report;

pub mod kernel_core {
    pub mod process_spawn {
        pub mod capsule_spawn {
            pub use crate::spawn_error::SpawnError;
        }
    }
}

pub mod security {
    pub mod capsule_manifest {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ManifestVerifyError {
            PayloadHash,
        }
    }
    pub mod nonos_id_cert {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum IdCertVerifyError {
            TrustAnchorPolicy,
        }
    }
}

pub mod sys {
    pub use crate::console::{boot_log, serial};
}

/// What `super::capsule_boot` names from the report, which sits at the root here.
mod capsule_boot {
    pub(crate) use crate::failures::failed;
}

#[cfg(test)]
mod count_tests;
#[cfg(test)]
mod source;
#[cfg(test)]
mod source_tests;
