// NONOS Operating System (AGPL-3.0-or-later)
// The two table encodings from src/arch/x86_64/iommu/tables. Both are
// self-contained: neither reads anything through `crate` or `super`.
#[path = "../../../../../../../../src/arch/x86_64/iommu/tables/sl_pte.rs"]
pub mod sl_pte;

#[path = "../../../../../../../../src/arch/x86_64/iommu/tables/context.rs"]
pub mod context;
