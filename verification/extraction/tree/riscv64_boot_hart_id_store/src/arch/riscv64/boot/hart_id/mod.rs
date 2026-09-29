// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/riscv64/boot/hart_id/state.rs"]
pub mod state;

#[path = "../../../../../../../../../src/arch/riscv64/boot/hart_id/store.rs"]
pub mod store;

pub use store::store;
