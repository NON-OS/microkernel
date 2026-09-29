// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/riscv64/fpu/context.rs"]
pub mod context;

#[path = "../../../../../../../../src/arch/riscv64/fpu/slot.rs"]
pub mod slot;

pub use context::FpContext;
pub use slot::FpSlot;
