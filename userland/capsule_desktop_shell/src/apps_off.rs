/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The apps first-boot setup turned off, as the kernel holds them. The kernel
 * spawns none of them, at boot or for a click, so every way in (dock,
 * Launchpad, the Go menu) shows each as off and says why it opens nothing.
 */

mod dim;
mod mask;
mod open;

pub use dim::dim;
pub use mask::is_off;
pub use open::{open, request, toast_failed};
