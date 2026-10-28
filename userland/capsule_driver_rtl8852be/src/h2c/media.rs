// NONOS Operating System (AGPL-3.0-or-later)

//! The station's role as the firmware keeps it (Linux v6.12 rtw89 `fw.c`):
//! `rtw89_fw_h2c_role_maintain` and `rtw89_fw_h2c_join_info`, for the one
//! station interface this driver has: MAC ID 0 on port 0 of band 0, WMM
//! set 0, a client (self role 0) of role "station" (1), never HE.

use super::packet::Cmd;
use super::send::{h2c, SendFail};
use super::Fw;
use crate::dma::DmaMem;
use crate::regs::{field, Mmio};
use crate::ring::tx::TxRing;

pub const ROLE_MAINTAIN: Cmd = Cmd { cat: 1, class: 8, func: 4 };
pub const JOIN_INFO: Cmd = Cmd { cat: 1, class: 8, func: 0 };
pub const MAC_ID: u8 = 0;
pub const PORT: u8 = 0;
const SELF_ROLE_CLIENT: u32 = 0;
const WIFI_ROLE_STATION: u32 = 1;

/// `enum rtw89_upd_mode`: the interface is created once and kept, so only
/// CREATE (0) is sent; rtw89's REMOVE and TYPE_CHANGE are not needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upd {
    Create = 0,
}

/// `enum rtw89_net_type`: no link before association, infrastructure after.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Net {
    NoLink = 0,
    Infra = 2,
}

pub fn role_body(upd: Upd) -> [u8; 4] {
    let w = field(0xff, MAC_ID as u32)
        | field(0x300, SELF_ROLE_CLIENT)
        | field(0x1c00, upd as u32)
        | field(0x1_e000, WIFI_ROLE_STATION);
    w.to_le_bytes()
}

pub fn join_body(dis_conn: bool, net: Net) -> [u8; 4] {
    let w = field(0xff, MAC_ID as u32)
        | field(0x100, dis_conn as u32)
        | field(0xe0_0000, PORT as u32)
        | field(0x300_0000, net as u32)
        | field(0x3c00_0000, WIFI_ROLE_STATION)
        | field(0xc000_0000, SELF_ROLE_CLIENT);
    w.to_le_bytes()
}

pub fn role_maintain<M: DmaMem>(
    r: &impl Mmio,
    q: &mut TxRing<M>,
    fw: &mut Fw,
    upd: Upd,
) -> Result<(), SendFail> {
    h2c(r, q, fw, ROLE_MAINTAIN, &role_body(upd), true)
}

pub fn join_info<M: DmaMem>(
    r: &impl Mmio,
    q: &mut TxRing<M>,
    fw: &mut Fw,
    dis_conn: bool,
    net: Net,
) -> Result<(), SendFail> {
    h2c(r, q, fw, JOIN_INFO, &join_body(dis_conn, net), true)
}
