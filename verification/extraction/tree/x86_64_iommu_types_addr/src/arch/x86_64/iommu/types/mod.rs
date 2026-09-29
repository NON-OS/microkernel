// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/iommu/types/addr.rs"]
pub mod addr;

#[path = "../../../../../../../../../src/arch/x86_64/iommu/types/limits.rs"]
pub mod limits;

pub use addr::IoVirtAddr;
pub use limits::{MAX_VTD_DEVICES, MAX_VTD_DOMAINS, MAX_VTD_MAPPINGS_PER_DOMAIN, PAGE_MASK_4K, PAGE_SHIFT_4K, PAGE_SIZE_4K};
