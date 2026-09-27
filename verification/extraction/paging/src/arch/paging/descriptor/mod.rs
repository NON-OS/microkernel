// NONOS Operating System (AGPL-3.0-or-later)
// The neutral flag vocabulary and both backends that read it.
#[path = "../../../../../../../src/arch/paging/descriptor/flags.rs"]
pub mod flags;

#[path = "../../../../../../../src/arch/paging/descriptor/x86_64.rs"]
pub mod x86_64;

pub mod aarch64;
