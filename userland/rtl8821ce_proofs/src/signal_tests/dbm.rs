// NONOS Operating System (AGPL-3.0-or-later)
//! The dBm rtw88 reads from the 8821C's PHY status: CCK from the LNA gain
//! table of the board's front end less twice the VGA index, OFDM from the
//! power less 110, and nothing for a page it does not read.

use crate::rx::signal::signal_dbm;

// A CCK status word 3 for LNA index `lna` and VGA index `vga`.
fn cck(lna: u32, vga: u32) -> [u32; 4] {
    [0, 0, 0, (lna >> 3) << 23 | (lna & 7) << 13 | vga << 8]
}

#[test]
fn cck_reads_the_gain_table_of_the_front_end() {
    assert_eq!(signal_dbm(cck(1, 10), 0), Some(8 - 20), "table 0, index 1");
    assert_eq!(signal_dbm(cck(9, 3), 2), Some(-24 - 6), "table 1, index 9: the high bit");
    assert_eq!(signal_dbm(cck(9, 3), 0), Some(-120), "past table 0's end");
    assert_eq!(signal_dbm(cck(7, 31), 0), Some(-52 - 62), "the weakest");
}

#[test]
fn ofdm_reads_the_power_less_110_within_the_floor() {
    let ofdm = |pwdb: u32| [1 | pwdb << 8, 0, 0, 0];
    assert_eq!(signal_dbm(ofdm(60), 0), Some(-50));
    assert_eq!(signal_dbm(ofdm(0), 1), Some(-110));
    assert_eq!(signal_dbm(ofdm(255), 0), Some(127), "no wrap");
    assert_eq!(signal_dbm([2, 0, 0, 0], 0), None, "page 2");
}
