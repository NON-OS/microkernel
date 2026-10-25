// NONOS Operating System (AGPL-3.0-or-later)
//! The net.core serve loop's wait cadence.
//!
//! This decides how often an idle networking stack wakes the scheduler, which
//! is the difference between a machine that looks idle and one that reports
//! most of a core in use.

mod attention;
mod decay;

use core::sync::atomic::AtomicU32;

fn fresh() -> AtomicU32 {
    AtomicU32::new(0)
}
