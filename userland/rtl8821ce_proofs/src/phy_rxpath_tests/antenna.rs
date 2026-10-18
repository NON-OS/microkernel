// NONOS Operating System (AGPL-3.0-or-later)

//! The receive path: its reset, the antenna switch, the filter.

use super::model::{Dev, REG_ACBB0, REG_CCK_CHECK, REG_RFECTL, REG_RXPSEL, RX_PSEL_RST};
use crate::phy::channel::Bw;
use crate::phy::rxpath::{post_tables, pre_tables, rfe_is_btg, set_channel};

#[test]
fn pre_tables_clears_and_post_tables_sets_the_rx_path_reset() {
    let dev = Dev::new();
    dev.seed(REG_RXPSEL, 0xFFFF_FFFF);
    pre_tables(&dev);
    assert_eq!(dev.get(REG_RXPSEL) & RX_PSEL_RST, 0, "rx-path reset cleared before tables");
    post_tables(&dev);
    assert_eq!(
        dev.get(REG_RXPSEL) & RX_PSEL_RST,
        RX_PSEL_RST,
        "rx-path reset released after tables"
    );
}

#[test]
fn set_channel_routes_the_antenna_and_selects_the_receive_path() {
    let dev = Dev::new();
    // A standard single-antenna board: WiFi-only (WLG) routing.
    set_channel(&dev, 1, Bw::W20, false);

    // The RFE control routes to the WiFi path: WL/WLG switch bits set, the
    // Bluetooth, control and WLA bits clear.
    let rfe = dev.get(REG_RFECTL);
    assert_eq!(rfe & ((1 << 20) | (1 << 22) | (1 << 21)), (1 << 20) | (1 << 22) | (1 << 21));
    assert_eq!(rfe & ((1 << 16) | (1 << 18) | (1 << 23)), 0, "bluetooth/wla path off");

    // The 2.4GHz receive path is selected and CCK reception is enabled.
    assert_eq!(dev.get(REG_RXPSEL) & (1 << 28), 1 << 28, "cck receive path selected");
    assert_eq!(dev.get(REG_CCK_CHECK) & (1 << 7), 0, "cck reception enabled");

    // The receive digital filter is programmed for 20MHz.
    assert_eq!((dev.get(REG_ACBB0) >> 28) & 0x3, 0x2, "rx digital filter set");
}

#[test]
fn a_bluetooth_shared_board_routes_the_bluetooth_path() {
    let dev = Dev::new();
    set_channel(&dev, 6, Bw::W20, true);
    let rfe = dev.get(REG_RFECTL);
    assert_eq!(rfe & (1 << 16), 1 << 16, "bluetooth-shared antenna path selected");
    assert_eq!(rfe & ((1 << 20) | (1 << 22)), 0, "wifi-only bits cleared");
}

#[test]
fn rfe_option_selects_the_antenna_path() {
    // The shared-antenna options route through Bluetooth; everything else is
    // WiFi-only. A standard single-antenna module is option 0.
    for opt in [2u8, 4, 7, 0x0A, 0x0C, 0x0F] {
        assert!(rfe_is_btg(opt), "option {opt:#x} is bluetooth-shared");
    }
    for opt in [0u8, 1, 3, 5, 6, 8] {
        assert!(!rfe_is_btg(opt), "option {opt:#x} is wifi-only");
    }
}
