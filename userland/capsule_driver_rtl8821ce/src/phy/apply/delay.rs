// NONOS Operating System (AGPL-3.0-or-later)

//! The tables' settle delays.

/// The delay a BB-table address in 0xF9..=0xFE stands for, as rtw88
/// `rtw_phy_cfg_bb` takes it: 0xFE 50 ms, 0xFD 5 ms, 0xFC 1 ms, and 0xFB to
/// 0xF9 microseconds, which the clock's tick covers. `None`: a register.
pub(super) fn bb_delay_ms(addr: u32) -> Option<u64> {
    match addr {
        0xFE => Some(50),
        0xFD => Some(5),
        0xF9..=0xFC => Some(1),
        _ => None,
    }
}

/// The delay an RF-table address stands for (rtw88 `rtw_phy_cfg_rf`): 0xFFE
/// 50 ms, 0xFE 100 us, which the clock's tick covers.
pub(super) fn rf_delay_ms(addr: u32) -> Option<u64> {
    match addr {
        0xFFE => Some(50),
        0xFE => Some(1),
        _ => None,
    }
}

/// The microsecond rtw88 lets each RF write settle: below the clock's tick,
/// so a short spin. A delay, not a wait for anything.
pub(super) fn rf_write_settle() {
    for _ in 0..4096 {
        core::hint::spin_loop();
    }
}
