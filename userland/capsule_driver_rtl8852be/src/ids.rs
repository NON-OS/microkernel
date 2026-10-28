// NONOS Operating System (AGPL-3.0-or-later)

//! The PCI ids the driver takes, and the register window it needs. Pure,
//! so the kernel's proofs can hold its inventory and claim scope to them.

/// PCI vendor: Realtek.
pub const PCI_VENDOR_REALTEK: u16 = 0x10ec;
/// The RTL8852BE's device ids, as Linux v6.12 rtw89 binds them
/// (`rtw8852be.c`, `rtw89_8852be_id_table`): 0xB852 and 0xB85B.
pub const RTL8852BE_DEVICE_IDS: [u16; 2] = [0xb852, 0xb85b];
/// The smallest register window the driver can use: the indirect access
/// window (`R_AX_INDIR_ACCESS_ENTRY`, `mac::macid`) sits at 0x40000, past
/// the PHY's at 0x10000. A BAR's size is a power of two, so that is 512 KiB
/// (the card's BAR is 1 MiB).
pub const BAR_MIN: u64 = 0x8_0000;
