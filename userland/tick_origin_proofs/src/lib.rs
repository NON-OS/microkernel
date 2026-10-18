// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for where a tick came from. The riscv64 trap frame and its
//! sstatus bits are the kernel's own files.

#![allow(dead_code)]

#[path = "../../../src/arch/riscv64/cpu/csr/status.rs"]
pub mod csr_status;
#[path = "../../../src/arch/riscv64/interrupts/frame/fields.rs"]
pub mod fields;
#[path = "../../../src/arch/riscv64/interrupts/frame/state.rs"]
pub mod state;

pub mod arch {
    pub mod riscv64 {
        pub mod cpu {
            pub mod csr {
                pub use crate::csr_status::*;
            }
        }
    }
}

#[cfg(test)]
mod level_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod reader_tests;
#[cfg(test)]
mod source;
