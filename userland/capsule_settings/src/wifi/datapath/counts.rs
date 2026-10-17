// NONOS Operating System (AGPL-3.0-or-later)

//! The data-path counts and diagnosis an RTL8821CE status reply carries.

/// The driver's TX and RX frame counts since bring-up, plus the number of
/// net_core link-protocol requests it has answered (zero means the stack never
/// reached the radio).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct DataPath {
    pub tx_ok: u32,
    pub tx_drop: u32,
    pub rx_ring: u32,
    pub rx_eth: u32,
    pub netif_reqs: u32,
    pub rx_err: u32,
    /// The last stalled efuse read, as named fields with no efuse content:
    /// flags (bit 0, the window read back all ones; bit 1, the control
    /// register held the address written), the address it stalled on plus
    /// one, and the LDO/efuse register's bank-select and LDO enable bits. All
    /// zero means no read has stalled since boot.
    pub efuse_flags: u32,
    pub efuse_stalled_at: u32,
    pub efuse_bank_ldo: u32,
    /// Where firmware bring-up stopped (0: it did not stop), the chunks that
    /// had landed, and `REG_MCUFW_CTRL` as the stop left it. Zero from a driver
    /// built before the report.
    pub fw_step: u8,
    pub fw_chunks: u32,
    pub fw_ctrl: u32,
}
