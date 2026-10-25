// NONOS Operating System (AGPL-3.0-or-later)

//! The receive chains, in RLC_CONFIG_CMD.

use super::context::{PHY_ID, RLC_CONFIG_LEN, RX_CHAIN_VALID_POS};

const RX_CHAIN_CNT_POS: u32 = 10;
const RX_CHAIN_MIMO_CNT_POS: u32 = 12;

/// The receive chain word for `valid_rx_ant` (a bitmap of antennas).
pub fn rx_chain_info(valid_rx_ant: u8) -> u32 {
    let chains: u32 = if valid_rx_ant.count_ones() >= 2 { 2 } else { 1 };
    (u32::from(valid_rx_ant & 0x7) << RX_CHAIN_VALID_POS)
        | (chains << RX_CHAIN_CNT_POS)
        | (chains << RX_CHAIN_MIMO_CNT_POS)
}

/// `iwl_rlc_config_cmd` for PHY 0.
pub fn rlc_config(valid_rx_ant: u8) -> [u8; RLC_CONFIG_LEN] {
    let mut c = [0u8; RLC_CONFIG_LEN];
    c[0..4].copy_from_slice(&PHY_ID.to_le_bytes());
    c[4..8].copy_from_slice(&rx_chain_info(valid_rx_ant).to_le_bytes());
    c
}
