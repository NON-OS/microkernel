// NONOS Operating System (AGPL-3.0-or-later)
// The error enum and the type layouts the checks below read, then the three
// checks themselves. `types` is mirrored whole through its own mod.rs, so its
// forty-six files come in at the paths they use to find each other.
#[path = "../../../../../src/elf/errors/mod.rs"]
pub mod errors;

#[path = "../../../../../src/elf/types/mod.rs"]
pub mod types;

pub mod loader;
pub mod reloc;
