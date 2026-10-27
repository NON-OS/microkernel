// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_mac_init`'s first part: TX queue control, SIFS, rate fallback
//! and protocol configuration (rtw8822c.c:2011-2053).

use crate::mac::op::MacOp;

pub(super) const LEN: usize = 32;

/// value8 |= BIT(7) & ~BIT(1) & ~BIT(2), which is BIT(7).
const TXQ_CTRL_SET: u8 = 1 << 7;
/// pre_txcnt = WLAN_PRE_TXCNT_TIME_TH (0x1E0) | BIT_EN_PRECNT (BIT(11)) = 0x09E0.
const PRE_TXCNT: u16 = 0x1E0 | 1 << 11;

pub(super) const OPS: &[MacOp; LEN] = &[
    MacOp::set8(0x0420, TXQ_CTRL_SET),      // REG_FWHW_TXQ_CTRL
    MacOp::write8(0x0421, 0x1F),            // + 1, WLAN_TXQ_RPT_EN
    MacOp::write16(0x0428, 0x100A),         // REG_SPEC_SIFS: CCK 0x0A | OFDM 0x10 << 8
    MacOp::write32(0x0514, 0x100A_0E0A),    // REG_SIFS: 0x0A | 0x0E<<8 | 0x0A<<16 | 0x10<<24
    MacOp::write16(0x063C, 0x0A0A),         // REG_RESP_SIFS_CCK: CTX 0x0A | IRX 0x0A << 8
    MacOp::write16(0x063E, 0x0E0E),         // REG_RESP_SIFS_OFDM: CTX 0x0E | IRX 0x0E << 8
    MacOp::write32(0x0430, 0x0100_0000),    // REG_DARFRC: DATA_RATE_FB_CNT_1_4
    MacOp::write32(0x0434, 0x0807_0504),    // REG_DARFRCH: DATA_RATE_FB_CNT_5_8
    MacOp::write32(0x043C, 0x0807_0504),    // REG_RARFRCH: RTS_RATE_FB_CNT_5_8
    MacOp::write32(0x0444, 0xFE01_F010),    // REG_ARFR0
    MacOp::write32(0x0448, 0x4000_0000),    // REG_ARFRH0
    MacOp::write32(0x044C, 0x003F_F010),    // REG_ARFR1_V1
    MacOp::write32(0x0450, 0x4000_0000),    // REG_ARFRH1_V1
    MacOp::write32(0x049C, 0x0600_F010),    // REG_ARFR4
    MacOp::write32(0x04A0, 0x4000_03E0),    // REG_ARFRH4
    MacOp::write32(0x04A4, 0x0600_F015),    // REG_ARFR5
    MacOp::write32(0x04A8, 0x0000_00E0),    // REG_ARFRH5
    MacOp::write8(0x0455, 0x70),            // REG_AMPDU_MAX_TIME_V1
    MacOp::set8(0x045E, 1 << 2),            // REG_TX_HANG_CTRL |= BIT_EN_EOF_V1
    MacOp::write8(0x04E5, PRE_TXCNT as u8), // REG_PRECNT_CTRL
    MacOp::write8(0x04E6, (PRE_TXCNT >> 8) as u8),
    // REG_PROT_MODE_CTRL: RTS_LEN_TH 0xFF | RTS_TX_TIME_TH 0x08 << 8
    // | MAX_AGG_PKT_LIMIT 0x3F << 16 | RTS_MAX_AGG_PKT_LIMIT 0x3F << 24.
    MacOp::write32(0x04C8, 0x3F3F_08FF),
    MacOp::write16(0x04CE, 0x0801), // REG_BAR_MODE_CTRL + 2: 0x01 | 0x08 << 8
    MacOp::write8(0x1448, 0x06),    // REG_FAST_EDCA_VOVI_SETTING: VO
    MacOp::write8(0x144A, 0x06),    // + 2: VI
    MacOp::write8(0x144C, 0x06),    // REG_FAST_EDCA_BEBK_SETTING: BE
    MacOp::write8(0x144E, 0x06),    // + 2: BK
    MacOp::clear8(0x0426, 1 << 5),  // REG_LIFETIME_EN: BIT_BA_PARSER_EN off
    MacOp::clear32(0x0440, 0x0060_0000), // REG_RRSR: BITS_RRSR_RSC off
    MacOp::write32(0x0500, 0x002F_A226), // REG_EDCA_VO_PARAM
    MacOp::write32(0x0504, 0x005E_A328), // REG_EDCA_VI_PARAM
    MacOp::write32(0x0508, 0x005E_A42B), // REG_EDCA_BE_PARAM
];
