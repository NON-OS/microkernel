// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/vga/console.rs"]
pub mod console;

#[path = "../../../../../../../../src/arch/x86_64/vga/constants.rs"]
pub mod constants;

pub mod ops;

#[path = "../../../../../../../../src/arch/x86_64/vga/state.rs"]
pub mod state;

pub use console::Console;
pub use constants::*;
