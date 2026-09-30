// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/multiboot/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../src/arch/x86_64/multiboot/header.rs"]
pub mod header;

pub use constants::{memory_type, tag, MULTIBOOT2_ARCHITECTURE_I386, MULTIBOOT2_BOOTLOADER_MAGIC, MULTIBOOT2_HEADER_MAGIC};
pub use header::{Multiboot2Header, Multiboot2Info, TagHeader};
