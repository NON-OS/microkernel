// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/gdt/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../src/arch/x86_64/gdt/entry_base.rs"]
pub mod entry_base;

#[path = "../../../../../../../../src/arch/x86_64/gdt/entry_presets.rs"]
pub mod entry_presets;

pub use constants::*;
