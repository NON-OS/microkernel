// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/iommu/regs/offsets/invalidate.rs"]
pub mod invalidate;

pub use invalidate::{iotlb_offset, iva_offset, CCMD, CCMD_CIRG_GLOBAL, CCMD_ICC, IOTLB_IAIG_MASK, IOTLB_IIRG_GLOBAL, IOTLB_IVT};
