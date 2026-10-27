// NONOS Operating System (AGPL-3.0-or-later)

//! The claim, the command register and the register window.

pub mod claim_note;
mod master;
mod run;

pub use master::{command, release};
pub use run::{run, Claimed};
