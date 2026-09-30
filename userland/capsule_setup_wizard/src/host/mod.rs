/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The computer-name step: the host the kernel and the Terminal name this
 * machine by, shown as name@host.
 */

mod rules;
mod state;

pub use rules::{check, Refused};
pub use state::HostState;
