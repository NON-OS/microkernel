// NONOS Operating System (AGPL-3.0-or-later)

pub mod stats;

pub use stats::{get_stats, get_stats_tuple, increment_exceptions, increment_keyboard, increment_mouse, increment_page_faults, increment_syscalls, increment_timer, reset_stats, InterruptCounters, InterruptStats, COUNTERS};
