// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/userspace/init/spawn_plan/desktop_fleet/spawn_gui_core.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod userspace;

pub fn spawn_gui_core() {
    crate::userspace::init::spawn_plan::desktop_fleet::spawn_gui_core::spawn_gui_core()
}

