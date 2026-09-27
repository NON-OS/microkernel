// NONOS Operating System (AGPL-3.0-or-later)
// The paging-depth decoding from src/arch/x86_64/iommu/regs/cap. This is the
// only producer of the level the table walk indexes by, so it belongs in the
// same extraction as the indexing arithmetic it has to keep in range.
#[path = "../../../../../../../../../src/arch/x86_64/iommu/regs/cap/agaw.rs"]
pub mod agaw;
