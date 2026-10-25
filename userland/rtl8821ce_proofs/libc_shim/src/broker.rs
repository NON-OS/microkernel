// NONOS Operating System (AGPL-3.0-or-later)

//! The device broker as the mounted bring-up files call it: every claim, map
//! and list refused, as on a machine without the card. The proofs build the
//! radio over host memory instead.

pub use crate::types::bar::BAR_KIND_MMIO;
pub use crate::types::bus::BUS_KIND_PCI;
pub use crate::types::device::DeviceRecord;
pub use crate::types::dma::DmaMapOut;
pub use crate::types::mmio::MmioMapOut;

pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;
pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;
pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;

pub fn mk_device_list(_class: u32, _buf: *mut DeviceRecord, _count: u64) -> i64 {
    0
}
pub fn mk_device_claim(_device_id: u64) -> i64 {
    -1
}
pub fn mk_device_release(_device_id: u64) -> i64 {
    -1
}
pub fn mk_mmio_map(
    _d: u64,
    _e: u64,
    _b: u32,
    _f: u32,
    _o: u64,
    _l: u64,
    _out: *mut MmioMapOut,
) -> i64 {
    -1
}
pub fn mk_pci_config_write(_d: u64, _e: u64, _offset: u32, _value: u16) -> i64 {
    -1
}
pub fn mk_dma_map(_d: u64, _e: u64, _l: u64, _f: u32, _out: &mut DmaMapOut) -> i64 {
    -1
}
pub fn mk_dma_unmap(_grant_id: u64) -> i64 {
    -1
}
