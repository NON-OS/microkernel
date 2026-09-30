/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The installer as the whole screen, in first-boot setup's look: setup's
 * panel on the left with the steps, the screen on the right, so the
 * install reads as the rest of the setup it follows.
 */

mod chrome;
mod frame;
mod palette;
mod subtitle;

pub use frame::paint;
pub use palette::BACKDROP;
