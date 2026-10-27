// NONOS Operating System (AGPL-3.0-or-later)

//! The PCI ids the driver takes. Pure, so the kernel's proofs can hold its
//! inventory to them.

/// PCI vendor: Realtek.
pub const PCI_VENDOR_REALTEK: u16 = 0x10EC;

/// The RTL8822CE's device ids, as Linux v6.12 rtw88 binds them
/// (`rtw8822ce.c`, `rtw_8822ce_id_table`): 0xC822 and 0xC82F.
pub const RTL8822CE_DEVICE_IDS: [u16; 2] = [0xC822, 0xC82F];
