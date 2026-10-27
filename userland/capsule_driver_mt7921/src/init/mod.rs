/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The MAC brought up once the firmware runs, in Linux v6.12's order:
//! the card's capabilities (`mt7921_run_firmware`), the efuse in buffer
//! mode (`mt7921_mcu_set_eeprom`), the MAC's receive length with header
//! translation and A-MSDU splitting left off (`mt7921_mac_init` turns them
//! on; the shared station takes whole 802.11 frames), the band's DMA
//! length with group-5 rate reports off (`mt792x_mac_init_band`), then
//! `__mt7921_start`: the MAC enabled, the channel domain, the receive path,
//! the transmit power limits (`power`). The CLC tables and SAR limits are
//! not sent: the firmware runs on its defaults, as Linux's `disable_clc`.

pub mod caps;
pub mod chan;
pub mod domain;
pub mod power;

use caps::Caps;

use crate::dma::Dma;
use crate::hw::addr::*;
use crate::hw::Regs;
use crate::mcu::cmd::*;
use crate::mcu::Mcu;

const DMA_DCR0: u32 = 0x820e7000;
const DMA_DCR0_MAX_RX_LEN: u32 = 0xfff8;
const DMA_DCR0_RXD_G5_EN: u32 = 1 << 23;
const MAX_RX_LEN: u32 = 1536;

/// Which step stopped the MAC's bring-up; its number is in the log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitStep {
    Caps = 1,
    Efuse = 2,
    MacEnable = 3,
    Domain = 4,
    RxPath = 5,
    TxPower = 6,
}

pub fn run(m: &mut Mcu, r: &Regs, d: &mut Dma) -> Result<Caps, InitStep> {
    let reply = m.send(r, d, GET_NIC_CAPAB, &[], true).map_err(|_| InitStep::Caps)?;
    let caps = caps::parse(reply.body());
    // EE_MODE_EFUSE, EE_FORMAT_WHOLE, no buffer.
    m.send(r, d, EFUSE_BUFFER_MODE, &[0, 1, 0, 0], true).map_err(|_| InitStep::Efuse)?;
    r.rmw_field(MDP_DCR1, MDP_DCR1_MAX_RX_LEN, MAX_RX_LEN);
    r.clear(MDP_DCR0, MDP_DCR0_DAMSDU_EN | MDP_DCR0_RX_HDR_TRANS_EN);
    r.rmw_field(DMA_DCR0, DMA_DCR0_MAX_RX_LEN, MAX_RX_LEN);
    r.clear(DMA_DCR0, DMA_DCR0_RXD_G5_EN);
    // Band 0 on.
    m.send(r, d, MAC_INIT_CTRL, &[1, 0, 0, 0], true).map_err(|_| InitStep::MacEnable)?;
    let dom = domain::body(caps);
    m.send(r, d, SET_CHAN_DOMAIN, &dom, false).map_err(|_| InitStep::Domain)?;
    let path = chan::body(1, 0, caps);
    m.send(r, d, SET_RX_PATH, &path, true).map_err(|_| InitStep::RxPath)?;
    for b in power::bodies(caps) {
        m.send(r, d, SET_RATE_TX_POWER, &b, false).map_err(|_| InitStep::TxPower)?;
    }
    Ok(caps)
}
