// NONOS Operating System (AGPL-3.0-or-later)
//! The SMMU record types alone: the finder that fills them from a tree is
//! fdt_proofs' to prove.
#[path = "../../../../../../../src/arch/fdt/find/smmu/info.rs"]
mod info;
#[path = "../../../../../../../src/arch/fdt/find/smmu/map.rs"]
mod map;

pub use info::SmmuInfo;
pub use map::{IommuMap, MapEntry, MAX_MAP_ENTRIES};
