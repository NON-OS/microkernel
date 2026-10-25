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

/* A `linux --box` run: its report taken only under the key the terminal
 * names, the argv that names it, and the capsule's own frames of it. */
#[path = "../../capsule_terminal/src/term/terminal/agentproof/job.rs"]
pub mod agentproof_job;
#[path = "../../capsule_terminal/src/jobs/box_hold.rs"]
pub mod box_hold;
#[path = "../../capsule_terminal/src/jobs/box_log.rs"]
pub mod box_log;
#[path = "../../capsule_terminal/src/jobs/box_missing.rs"]
pub mod box_missing;
#[path = "../../capsule_terminal/src/jobs/box_screen.rs"]
pub mod box_screen;
#[path = "../../capsule_linux/src/linux/boxed/frames.rs"]
pub mod linux_frames;
#[path = "../../capsule_terminal/src/command/builtin/tool_argv.rs"]
pub mod tool_argv;
#[path = "../../capsule_terminal/src/command/builtin/tool_service.rs"]
pub mod tool_service;

/* A plain `linux` run asking to start again with the network: once, and
 * handed its input again from the start. */
#[cfg(test)]
#[path = "../../capsule_terminal/src/jobs/net_ask.rs"]
pub mod net_ask;
#[cfg(test)]
#[path = "../../capsule_terminal/src/jobs/stdin_kept.rs"]
pub mod stdin_kept;
#[cfg(test)]
#[path = "../../libc/src/linux_ask_net.rs"]
pub mod linux_ask_net;

#[cfg(test)]
mod agentproof_job_sweep_tests;
#[cfg(test)]
mod agentproof_job_tests;
#[cfg(test)]
mod box_argv_tests;
#[cfg(test)]
mod box_glue_tests;
#[cfg(test)]
mod box_log_tests;
#[cfg(test)]
mod box_missing_tests;
#[cfg(test)]
mod box_net_tests;
#[cfg(test)]
mod box_service_tests;
#[cfg(test)]
mod box_screen_left;
#[cfg(test)]
mod box_screen_tests;
#[cfg(test)]
mod box_stop_tests;
#[cfg(test)]
mod box_sweep_tests;
#[cfg(test)]
mod box_tests;
#[cfg(test)]
mod box_wait_tests;
#[cfg(test)]
mod install_follow_tests;
#[cfg(test)]
mod kill_target_tests;
#[cfg(test)]
mod net_ask_tests;
#[cfg(test)]
mod pkg_handoff_tests;
#[cfg(test)]
mod tool_busy_tests;
