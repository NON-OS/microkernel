// NONOS Operating System (AGPL-3.0-or-later)

//! The claim, the command register and the register window.

pub mod claim_note;
mod give_back;
mod master;
mod run;

pub use give_back::give_back;
pub use master::command;
pub use run::run;
