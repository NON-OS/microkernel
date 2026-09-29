// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/aarch64/fpu/context.rs"]
pub mod context;

#[path = "../../../../../../../../src/arch/aarch64/fpu/slot.rs"]
pub mod slot;

pub use context::FpSimdContext;
pub use slot::FpSimdSlot;
