// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Loading Realtek's patch into an RTL8821C Bluetooth controller on USB.
//! Its ROM answers the standard commands, but Linux v6.12 (btrtl.c) never
//! uses it unpatched: the patch is what the controller is meant to run.
//! Facts are cited as btrtl.c has them; the code is this crate's own.
//!
//! The steps, in `load::prepare`: a Realtek USB id (`ids`), then Read
//! Local Version naming the RTL8821C's ROM (`chip`), then its ROM version
//! by vendor command 0xfc6d, the patch for that version out of the
//! "Realtech" file (`epatch`, `project`) with the config after it
//! (`config`, `image`), and the whole by vendor command 0xfc20 in numbered
//! fragments (`fragments`). The controller runs the patch once the last
//! fragment is taken (btrtl.c reads its new version straight after), and
//! the host's own bring-up then begins with HCI Reset. A controller already
//! running a patch is left as it is; no other is sent a vendor command.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod chip;
pub mod config;
pub mod epatch;
pub mod error;
pub mod fragments;
pub mod ids;
pub mod image;
pub mod load;
pub mod project;

pub use error::RtlError;
pub use load::{prepare, Outcome, DOWNLOAD, READ_ROM_VERSION};
