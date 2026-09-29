// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/memory/paging/constants/align_funcs.rs"]
pub mod align_funcs;

#[path = "../../../../../../../../src/memory/paging/constants/page_sizes.rs"]
pub mod page_sizes;

#[path = "../../../../../../../../src/memory/paging/constants/pt_index.rs"]
pub mod pt_index;

pub use align_funcs::*;
pub use page_sizes::*;
pub use pt_index::*;
