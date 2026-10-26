// NONOS Operating System (AGPL-3.0-or-later)

//! The received signal in dBm from the PHY status, as rtw88 reads it on the
//! 8821C (`query_phy_status_page0`, `query_phy_status_page1`, `get_cck_rx_pwr`):
//! the signal power mac80211 reports for the frame.

/// The CCK LNA gains by index, for RF front-end option 0 and for the others.
const LNA_GAIN_0: [i8; 8] = [22, 8, -6, -22, -31, -40, -46, -52];
const LNA_GAIN_1: [i8; 16] =
    [10, 6, 2, -2, -6, -10, -14, -17, -20, -24, -28, -31, -34, -37, -40, -44];
/// rtw88's floor for a reading it cannot make.
const MIN_DBM: i8 = -120;

/// The frame's signal, or `None` for a PHY status page rtw88 does not read.
pub fn signal_dbm(phy: [u32; 4], rfe_option: u8) -> Option<i8> {
    match phy[0] & 0xF {
        0 => Some(cck(phy[3], rfe_option)),
        1 => {
            let pwdb = ((phy[0] >> 8) & 0xFF) as i16;
            Some((pwdb - 110).clamp(i16::from(MIN_DBM), i16::from(i8::MAX)) as i8)
        }
        _ => None,
    }
}

// The LNA gain less twice the VGA index; the LNA index is out of the table's
// range, the floor.
fn cck(w3: u32, rfe_option: u8) -> i8 {
    let vga = ((w3 >> 8) & 0x1F) as i16;
    let lna = (((w3 >> 23) & 1) << 3 | (w3 >> 13) & 0x7) as usize;
    let table: &[i8] = if rfe_option == 0 { &LNA_GAIN_0 } else { &LNA_GAIN_1 };
    table.get(lna).map_or(MIN_DBM, |&gain| (i16::from(gain) - 2 * vga) as i8)
}
