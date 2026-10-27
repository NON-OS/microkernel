/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The commands this driver sends the MCU, as the facts of Linux v6.12
//! `mt76_connac_mcu.h`: the plain download commands, the extended ones
//! (`MCU_EXT_CMD`, carried under `MCU_CMD_EXT_CID`), the unified ones
//! (`MCU_UNI_CMD`) and the connac offload ones (`MCU_CE_CMD`).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Plain,
    Ext,
    Uni,
    Ce,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd {
    pub kind: Kind,
    pub id: u8,
    pub query: bool,
}

const fn cmd(kind: Kind, id: u8) -> Cmd {
    Cmd { kind, id, query: false }
}

pub const EXT_CID: u8 = 0xed;

pub const TARGET_ADDRESS_LEN_REQ: Cmd = cmd(Kind::Plain, 0x01);
pub const FW_START_REQ: Cmd = cmd(Kind::Plain, 0x02);
pub const PATCH_START_REQ: Cmd = cmd(Kind::Plain, 0x05);
pub const PATCH_FINISH_REQ: Cmd = cmd(Kind::Plain, 0x07);
pub const PATCH_SEM_CONTROL: Cmd = cmd(Kind::Plain, 0x10);

pub const EFUSE_BUFFER_MODE: Cmd = cmd(Kind::Ext, 0x21);
pub const MAC_INIT_CTRL: Cmd = cmd(Kind::Ext, 0x46);
pub const SET_RX_PATH: Cmd = cmd(Kind::Ext, 0x4e);

pub const DEV_INFO_UPDATE: Cmd = cmd(Kind::Uni, 0x01);
pub const BSS_INFO_UPDATE: Cmd = cmd(Kind::Uni, 0x02);
pub const STA_REC_UPDATE: Cmd = cmd(Kind::Uni, 0x03);
pub const ROC: Cmd = cmd(Kind::Uni, 0x27);

pub const SET_RX_FILTER: Cmd = cmd(Kind::Ce, 0x0a);
pub const SET_CHAN_DOMAIN: Cmd = cmd(Kind::Ce, 0x0f);
pub const GET_NIC_CAPAB: Cmd = cmd(Kind::Ce, 0x8a);

impl Cmd {
    /// Whether the reply carries the unified event (`cid`, `status`) that
    /// `mt7921_mcu_parse_response` reads the outcome from.
    pub fn uni_status(self) -> bool {
        self.kind == Kind::Uni && matches!(self.id, 0x01..=0x03 | 0x05..=0x07)
    }
}
