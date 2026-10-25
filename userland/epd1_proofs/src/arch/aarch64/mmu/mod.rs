// NONOS Operating System (AGPL-3.0-or-later)
//! The attribute encoder, the trampoline's geometry and its leaves'
//! attributes, all unmodified. `attributes` restates the
//! kernel's `attributes.rs`: a `#[path]` directory module needs its children
//! named here.

#[path = "../../../../../../src/arch/aarch64/mmu/attributes"]
pub mod attributes {
    mod constructors;
    mod descriptor;
    mod kind;
    mod page;
    mod pte;

    pub use kind::MemoryType;
    pub use page::PageAttributes;
    pub use pte::*;
}

#[path = "../../../../../../src/arch/aarch64/mmu/trampoline/geometry.rs"]
pub mod geometry;
#[path = "../../../../../../src/arch/aarch64/mmu/trampoline/leaves.rs"]
pub mod leaves;
