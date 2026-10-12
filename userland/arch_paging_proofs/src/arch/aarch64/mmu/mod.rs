// NONOS Operating System (AGPL-3.0-or-later)
//! The module path the aarch64 encoder reaches for its memory-type table,
//! reconstructed so the encoder is included unmodified and stays bound to the
//! same table MAIR_EL1 is built from. The boot tables' own encoder,
//! `PageAttributes`, comes with it, so both are held to one boundary.

// Restates the kernel's `attributes.rs`: a `#[path]` directory module needs
// its children named here.
#[path = "../../../../../../src/arch/aarch64/mmu/attributes"]
pub mod attributes {
    mod constructors;
    mod descriptor;
    mod kind;
    mod page;
    mod pte;

    pub use kind::MemoryType;
    pub use page::PageAttributes;
    pub use pte::{PTE_AP_RW_ALL, PTE_PXN, PTE_UXN};
}

pub use attributes::MemoryType;
