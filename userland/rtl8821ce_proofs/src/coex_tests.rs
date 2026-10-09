// NONOS Operating System (AGPL-3.0-or-later)
//! The Wi-Fi-only antenna grant against rtw88: the switch value for every
//! front-end option as rtw8821c_coex_cfg_ant_switch picks it, and the
//! registers take_antenna leaves on a modeled chip.

use crate::coex::switch::{has_switch, switch_to, AntSwitch};
use crate::coex::take_antenna;
use crate::pwr_mock::Chip;
use crate::regs::Mmio;

#[test]
fn the_switch_points_at_wifi_for_every_front_end() {
    // (rfe, position after the BTG override, REG_RFE_CTRL8[31:28]).
    let want = [
        (0, AntSwitch::Wlg, 2), (1, AntSwitch::Wlg, 2), (8, AntSwitch::Wlg, 2),
        (2, AntSwitch::WlgBt, 2), (10, AntSwitch::WlgBt, 3), (7, AntSwitch::WlgBt, 3),
        (15, AntSwitch::WlgBt, 3), (3, AntSwitch::Wlg, 1), (11, AntSwitch::Wlg, 1),
        (4, AntSwitch::Wla, 2), (12, AntSwitch::Wla, 2), (5, AntSwitch::Wlg, 2),
    ];
    for (rfe, pos, val) in want {
        assert_eq!(switch_to(rfe, AntSwitch::Wlg), (pos, val), "rfe {rfe}");
    }
}

fn granted(rfe: u8) -> Chip {
    let mut c = Chip::new(0xEA);
    c.stuck = Some((0x1703, 0x20)); // LTECOEX_READY
    assert!(take_antenna(&c, rfe), "the indirect window answered");
    c
}

#[test]
fn both_grants_go_through_lte_coex_ctrl() {
    let c = granted(0);
    let w = c.writes.borrow();
    let ctrl: Vec<u8> = w.iter().filter(|x| x.0 == 0x1700).map(|x| x.1).collect();
    assert_eq!(ctrl.iter().filter(|&&b| b == 0x38).count(), 8, "four reads, four writes of 0x38");
    let data = |i: usize| u32::from_le_bytes([w[i].1, w[i + 1].1, w[i + 2].1, w[i + 3].1]);
    let wdata: Vec<u32> = (0..w.len()).filter(|&i| w[i].0 == 0x1704).map(data).collect();
    assert_eq!(wdata, vec![0x4000, 0x0400, 0x3000, 0x0300], "BT low, WL high");
}

#[test]
fn wifi_owns_the_path_and_the_switch() {
    let c = granted(0);
    assert_eq!(c.read8(0x0073) & 0x04, 0x04, "BIT_LTE_MUX_CTRL_PATH");
    assert_eq!(c.read8(0x0CB4), 0x77, "DPDT_CTRL_PIN");
    assert_eq!(c.read8(0x0CB7) >> 4, 0x2, "switch to WLG");
    assert_eq!(c.read32(0x004C) & (3 << 23), 1 << 24, "DPDT_WL_SEL on, SEL_EN off");
    assert_eq!(c.read8(0x0067) & 0x30, 0x30, "BIT_CTRL_TYPE1 and 2");
    assert_eq!(c.read16(0x00AA), 0x8003, "scoreboard: active, on");
}

#[test]
fn boards_without_a_switch_keep_their_pins() {
    for rfe in [5, 6, 13, 14] {
        assert!(!has_switch(rfe), "rfe {rfe}");
        let c = granted(rfe);
        assert_eq!(c.read8(0x0CB4), 0, "RFE_CTRL8 untouched on rfe {rfe}");
    }
    assert!(has_switch(0) && has_switch(2) && has_switch(12));
}
