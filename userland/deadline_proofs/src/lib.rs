// NONOS Operating System (AGPL-3.0-or-later)
//! Host proof for the libc monotonic Deadline. A `time` module mirroring the
//! libc path supplies a settable stand-in for the uptime syscall, so the real
//! `deadline.rs` runs unchanged, failed clock reads included.

pub mod time;

#[cfg(test)]
mod clock_fail_tests;
#[cfg(test)]
mod tests;
