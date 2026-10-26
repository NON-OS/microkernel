// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for aarch64 idle states. The kernel's files are mounted at the
//! top, beside each other as in the kernel, so `super::format` resolves;
//! `arch` gives `select.rs` the `crate::arch` path it names its types by.

#[path = "../../../src/arch/aarch64/idle_states/format.rs"]
pub mod format;
#[path = "../../../src/arch/aarch64/idle_states/rules.rs"]
pub mod rules;
#[path = "../../../src/arch/aarch64/idle_states/select.rs"]
pub mod select;
#[path = "../../../src/arch/fdt/find/idle/types.rs"]
pub mod types;

pub mod arch {
    pub mod fdt {
        pub mod find {
            pub mod idle {
                pub use crate::types::{CpuStates, IdleState, MAX_PER_CPU, MAX_STATES};
            }
        }
    }
}

#[cfg(test)]
mod edge_tests;
#[cfg(test)]
mod format_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod owner_tests;
#[cfg(test)]
mod select_tests;
