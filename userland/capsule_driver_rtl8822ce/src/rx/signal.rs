// NONOS Operating System (AGPL-3.0-or-later)

//! The received signal in dBm from the 8822C's PHY status (rtw88
//! `query_phy_status_page0` and `_page1`, rtw8822c.c:2548-2671; the field
//! positions are rtw8822c.h's `GET_PHY_STAT_P0_*` and `_P1_*`). A CCK frame
//! (page 0) carries path A's power, corrected by two dB a step where its
//! gain index is outside the AGC table's bounds; an OFDM frame (page 1)
//! carries both paths', and the stronger is taken. Each is 110 above dBm.
//! rtw88 sums in 8 bits and lets a nonsense reading wrap; here the sum is
//! wide and clamped to rtw88's -120 floor and the i8 ceiling.

use core::sync::atomic::{AtomicU8, Ordering};

/// rtw88's floor for a reading it cannot make.
const MIN_DBM: i16 = -120;
/// The 6-bit gain index's range, the bounds until the PHY sets them.
static CCK_UPPER: AtomicU8 = AtomicU8::new(0x3F);
static CCK_LOWER: AtomicU8 = AtomicU8::new(0);

/// The CCK gain index bounds the PHY bring-up read from its tables
/// (`dm_info->cck_gi_u_bnd`, `cck_gi_l_bnd`).
pub fn set_cck_bounds(upper: u8, lower: u8) {
    CCK_UPPER.store(upper, Ordering::Relaxed);
    CCK_LOWER.store(lower, Ordering::Relaxed);
}

/// The frame's signal, or `None` for a PHY status page rtw88 does not read.
/// The front-end option the RTL8821CE's reading needs plays no part here.
pub fn signal_dbm(phy: [u32; 4], _rfe_option: u8) -> Option<i8> {
    let w0 = phy[0];
    let dbm = match w0 & 0xF {
        0 => cck((w0 >> 8) & 0xFF, (w0 >> 16) & 0x3F),
        1 => i16::max(((w0 >> 8) & 0xFF) as i16, ((w0 >> 16) & 0xFF) as i16) - 110,
        _ => return None,
    };
    Some(dbm.clamp(MIN_DBM, i16::from(i8::MAX)) as i8)
}

/// Path A's CCK power from its PWDB and gain index.
fn cck(pwdb: u32, gain: u32) -> i16 {
    let (u, l) = (
        u32::from(CCK_UPPER.load(Ordering::Relaxed)),
        u32::from(CCK_LOWER.load(Ordering::Relaxed)),
    );
    let mut power = pwdb as i16;
    if gain < l {
        power += ((l - gain) << 1) as i16;
    } else if gain > u {
        power -= ((gain - u) << 1) as i16;
    }
    power - 110
}
