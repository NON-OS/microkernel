/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The apps first-boot setup lets the person turn off, and how the choice
 * travels: a byte with one bit per app, a set bit meaning off. Zero is every
 * app on, which is what a record kept before this choice existed and a boot
 * that never finished setup both read as.
 *
 * Setup hands the byte to the kernel in its exit status, and the kernel then
 * spawns no turned-off app at boot or on demand. It keeps its own copy of the
 * bits in src/userspace/init/app_choice/bits.rs; the two must agree.
 */

mod bits;
mod exit;
mod table;

pub use bits::{BROWSER, CALCULATOR, EDITOR, FILES, LINUX, MEDIA, NONE_OFF, STORE, WALLET};
pub use exit::{exit_parts, exit_status, EXIT_SHIFT};
pub use table::{bit_for_service, is_off, App, OPTIONAL, REQUIRED};
