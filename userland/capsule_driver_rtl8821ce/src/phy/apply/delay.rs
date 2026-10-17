// NONOS Operating System (AGPL-3.0-or-later)

//! The tables' settle delays.

// A BB-table address in 0xF9..=0xFE is a settle delay, not a register.
pub(super) fn is_bb_delay(addr: u32) -> bool {
    (0xF9..=0xFE).contains(&addr)
}

// An RF-table address of 0xFE or 0xFFE is a settle delay.
pub(super) fn is_rf_delay(addr: u32) -> bool {
    addr == 0xFE || addr == 0xFFE
}

// A short busy-wait; the exact duration only affects settle time, not the
// register program the tables produce.
pub(super) fn settle() {
    for _ in 0..4096 {
        core::hint::spin_loop();
    }
}
