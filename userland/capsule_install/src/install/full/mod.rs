/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The installer as the whole screen, on an install boot: straight after
 * first-boot setup, with no window manager, shell or app running, drawn
 * through the compositor the way setup draws. The screens, the plan, the
 * write and the read-back are the window's own; only the frame and the way
 * keys arrive differ.
 */

mod active;
mod draw;
mod grab;
mod input;
mod peers;
mod run;
mod start;
mod surface;

pub use active::{active, setup_kept, wanted};
pub use run::run;
