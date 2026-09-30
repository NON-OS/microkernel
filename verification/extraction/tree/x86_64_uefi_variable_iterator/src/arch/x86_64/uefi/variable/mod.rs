// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/uefi/variable/iterator.rs"]
pub mod iterator;

#[path = "../../../../../../../../../src/arch/x86_64/uefi/variable/utils.rs"]
pub mod utils;

pub use iterator::VariableIterator;
pub use utils::{name_to_ucs2, ucs2_to_string};
