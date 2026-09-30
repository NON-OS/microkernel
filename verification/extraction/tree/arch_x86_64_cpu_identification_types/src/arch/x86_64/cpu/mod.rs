// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/cpu/identification_types.rs"]
pub mod identification_types;

#[path = "../../../../../../../../src/arch/x86_64/cpu/vendor.rs"]
pub mod vendor;

pub use vendor::CpuVendor;
