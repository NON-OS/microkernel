// NONOS Operating System (AGPL-3.0-or-later)
//! The join's firmware commands, byte for byte.
//!
//! Each image below is the structure its builder names (fw/api/phy-ctxt.h,
//! mac-cfg.h, time-event.h, datapath.h, scan.h of Linux v6.12), with every
//! field Linux fills for a legacy station join written out at its offset.
//! The versions are held to what the bundled firmware files report in their
//! command version table: a firmware listing another version of any of
//! these commands is refused before anything is sent.

mod contexts;
mod fixtures;
mod keys;
mod layouts;
mod link;
mod session;
