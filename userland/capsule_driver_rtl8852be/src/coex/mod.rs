// NONOS Operating System (AGPL-3.0-or-later)

//! Wi-Fi/Bluetooth coexistence, as far as a Wi-Fi-only system needs it:
//! `__rtw8852bx_btc_init_cfg` (Linux v6.12 rtw89 `rtw8852b_common.c`) and
//! then the WL-only antenna grant. rtw89's run-time coexistence engine and
//! its firmware reports are not ported; the grant never changes after this.

pub mod grant;
pub mod lte;
pub mod pta;

use crate::phy::bb::Bb;
use crate::regs::{Mmio, Rw};
pub use lte::LteBusy;

const R_BTC_BT_COEX_MSK_TABLE: u32 = 0xda30;
const B_BTC_PRI_MASK_TX_RESP_V1: u32 = 0x8;
const R_AX_WL_PRI_MSK: u32 = 0xda10;
const B_AX_PTA_WL_PRI_MASK_BCNQ: u32 = 0x100;
const R_BTC_BREAK_TABLE: u32 = 0xda2c;
const BTC_BREAK_PARAM: u32 = 0xf0ff_ffff;
const RR_WLSEL: u32 = 0x2;
const RR_LUTWE: u32 = 0xef;
const RR_LUTWA: u32 = 0x33;
const RR_LUTWD0: u32 = 0x3f;
const SS_GROUP: u32 = 0;
const TX_GROUP: u32 = 2;
const FULL: u32 = 0xfffff;

/// `rtw8852b_btc_set_rfe`: RFE 0 and odd RFE types share one antenna
/// between WL and BT; even ones have a third, dedicated antenna.
pub const fn shared_antenna(rfe: u8) -> bool {
    rfe == 0 || rfe % 2 == 1
}

/// The TRX mask values (path A SS, path B SS, path A TX, path B TX).
pub const fn trx_masks(rfe: u8) -> [u32; 4] {
    match shared_antenna(rfe) {
        true => [0x5ff, 0x5ff, 0x5ff, 0x55f],
        false => [0x5df, 0x5df, 0x5ff, 0x5ff],
    }
}

fn trx_mask(bb: &Bb<'_, impl Mmio>, path: u8, group: u32, val: u32) {
    bb.write_rf(path, RR_LUTWE, FULL, 0x20000);
    bb.write_rf(path, RR_LUTWA, FULL, group);
    bb.write_rf(path, RR_LUTWD0, FULL, val);
    bb.write_rf(path, RR_LUTWE, FULL, 0);
}

pub fn init(r: &impl Mmio, rfe: u8) -> Result<(), LteBusy> {
    pta::coex_init(r)?;
    r.set32(R_BTC_BT_COEX_MSK_TABLE, B_BTC_PRI_MASK_TX_RESP_V1);
    r.set32(R_AX_WL_PRI_MSK, B_AX_PTA_WL_PRI_MASK_BCNQ);
    let bb = Bb(r);
    bb.write_rf(0, RR_WLSEL, FULL, 0);
    bb.write_rf(1, RR_WLSEL, FULL, 0);
    let m = trx_masks(rfe);
    trx_mask(&bb, 0, SS_GROUP, m[0]);
    trx_mask(&bb, 1, SS_GROUP, m[1]);
    trx_mask(&bb, 0, TX_GROUP, m[2]);
    trx_mask(&bb, 1, TX_GROUP, m[3]);
    r.write32(R_BTC_BREAK_TABLE, BTC_BREAK_PARAM);
    pta::bt_counter_on(r);
    grant::wl_only(r)
}
