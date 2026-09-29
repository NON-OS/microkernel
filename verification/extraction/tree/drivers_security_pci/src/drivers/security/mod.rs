// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/drivers/security/constants.rs"]
pub mod constants;

#[path = "../../../../../../../src/drivers/security/error.rs"]
pub mod error;

#[path = "../../../../../../../src/drivers/security/pci.rs"]
pub mod pci;

pub use constants::{ASSUMED_CPU_FREQ_MHZ, DEFAULT_ADMIN_OPS_PER_SEC, DEFAULT_DMA_OPS_PER_SEC, DEFAULT_IO_OPS_PER_SEC, HIGH_MMIO_START, KERNEL_PHYS_END, LOW_MMIO_END, LOW_MMIO_START, MAX_DMA_SIZE, MAX_PHYS_ADDR_BITS, MAX_PRP_ENTRIES, PAGE_SIZE, PCI_CONFIG_SPACE_SIZE, PCI_EXTENDED_CONFIG_SIZE, PCI_MAX_BUS, PCI_MAX_DEVICE, PCI_MAX_FUNCTION, PLATFORM_MMIO_END, PLATFORM_MMIO_START, PROTECTED_CONFIG_OFFSETS, RATE_LIMIT_WINDOW_MS};
pub use error::DriverError;
pub use pci::{build_config_address, is_config_write_allowed, is_sensitive_config_read, validate_pci_access, validate_pci_extended_access};
