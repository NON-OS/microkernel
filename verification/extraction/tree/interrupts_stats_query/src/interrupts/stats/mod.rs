// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/interrupts/stats/counters.rs"]
pub mod counters;

#[path = "../../../../../../../src/interrupts/stats/query.rs"]
pub mod query;

pub use counters::{increment_exceptions, increment_keyboard, increment_mouse, increment_page_faults, increment_syscalls, increment_timer, InterruptCounters, COUNTERS};
pub use query::{get_stats, get_stats_tuple, reset_stats, InterruptStats};
