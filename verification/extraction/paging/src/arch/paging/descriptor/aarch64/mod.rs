// NONOS Operating System (AGPL-3.0-or-later)
// The aarch64 backend. `read.rs` reads its bit positions through `super::bits`
// and `build.rs` reads the neutral flags through `super::super::flags`, so all
// three files are mirrored and the module graph matches the kernel's.
#[path = "../../../../../../../../src/arch/paging/descriptor/aarch64/bits.rs"]
pub mod bits;

#[path = "../../../../../../../../src/arch/paging/descriptor/aarch64/read.rs"]
pub mod read;

#[path = "../../../../../../../../src/arch/paging/descriptor/aarch64/build.rs"]
pub mod build;

pub use build::{leaf, table};
pub use read::{address, is_block, is_executable, is_present, is_user, is_writable,
    table_grants_user};
