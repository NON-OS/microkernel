// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/memory/layout/manager/address.rs"]
pub mod address;

pub use address::{in_kernel_space, in_user_space, is_canonical, range, selfref_l4_va};
