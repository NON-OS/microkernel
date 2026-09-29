// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/riscv64/cpu/caps/bits.rs"]
pub mod bits;

#[path = "../../../../../../../../../src/arch/riscv64/cpu/caps/query.rs"]
pub mod query;

#[path = "../../../../../../../../../src/arch/riscv64/cpu/caps/state.rs"]
pub mod state;

pub use query::{has_a, has_c, has_d, has_f, has_v, is_configured};
