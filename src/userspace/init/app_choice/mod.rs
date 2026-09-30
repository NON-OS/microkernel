/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The apps first-boot setup turned off. Setup's exit status carries them;
 * init records them before it starts the desktop, and from then on no app
 * turned off is spawned, at boot or on demand. Only spawns are withheld:
 * every check an app that does start goes through is unchanged.
 */

mod bits;
mod gate;
mod names;
mod present;
mod profile;

#[cfg(feature = "microkernel-setup-wizard")]
pub(crate) use gate::choose;
pub(crate) use gate::off;
pub(crate) use names::{capsule_off, linux_off, tool_off, window_off};
pub(crate) use present::PRESENT;
pub(crate) use profile::apply as apply_profile;
