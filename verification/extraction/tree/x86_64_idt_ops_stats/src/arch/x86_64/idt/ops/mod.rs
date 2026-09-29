// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/idt/ops/stats.rs"]
pub mod stats;

pub use stats::{get_stats, get_vector_count, IdtStats};
