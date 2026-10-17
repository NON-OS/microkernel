// NONOS Operating System (AGPL-3.0-or-later)
//! Waits end on elapsed time, not on a count of reads, each step
//! within rtw88's budget for it, and a completion that lands only at the
//! deadline is still seen.

mod steps;
mod time;
