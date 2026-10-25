// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../src/smp/percpu/by_base.rs"]
pub mod by_base;
#[path = "../../../../src/smp/percpu/layout.rs"]
pub mod layout;
pub mod operations;
#[path = "../../../../src/smp/percpu/types.rs"]
pub mod types;
#[path = "../../../../src/smp/percpu/types_new.rs"]
mod types_new;

pub use types::PerCpuData;
