// NONOS Operating System (AGPL-3.0-or-later)
//! The programs the terminal starts, waits on and stops, and `pkg`'s worker answer.

#[path = "../../capsule_terminal/src/command/builtin/nox/kill_target.rs"]
pub mod kill_target;
#[path = "../../capsule_terminal/src/command/builtin/tool_busy.rs"]
pub mod tool_busy;

// A load `install` stopped waiting on, followed from the job's ticks.
#[path = "../../capsule_terminal/src/command/builtin/nox/install/follow.rs"]
pub mod install_follow;

// The answer a `pkg` worker thread hands back to the window thread.
#[path = "../../libc/src/thread/handoff.rs"]
pub mod pkg_handoff;

#[cfg(test)]
mod install_follow_tests;
#[cfg(test)]
mod kill_target_tests;
#[cfg(test)]
mod pkg_handoff_tests;
#[cfg(test)]
mod tool_busy_tests;
