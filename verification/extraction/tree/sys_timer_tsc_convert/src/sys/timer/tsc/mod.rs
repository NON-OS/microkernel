// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/sys/timer/tsc/consts.rs"]
pub mod consts;

#[path = "../../../../../../../../src/sys/timer/tsc/convert.rs"]
pub mod convert;

pub use consts::{BOOT_EPOCH_MS, BOOT_TSC, TIMER_INIT, TSC_FREQ_HZ};
pub use convert::{ms_to_ticks, ticks_to_ms, ticks_to_ns, ticks_to_us, tsc_frequency, us_to_ticks};
