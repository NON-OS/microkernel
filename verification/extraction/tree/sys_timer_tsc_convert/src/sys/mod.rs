// NONOS Operating System (AGPL-3.0-or-later)

pub mod timer;

pub use timer::{ms_to_ticks, ticks_to_ms, ticks_to_ns, ticks_to_us, tsc_frequency, us_to_ticks};
