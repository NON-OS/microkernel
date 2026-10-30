// NONOS Operating System (AGPL-3.0-or-later)

// The kernel clock is not mirrored. Time zero means no capsule report is ever
// fresh, so the syscall answers from the table scan, as it does at boot.
pub mod clock {
    pub fn since_boot_ms() -> u64 {
        0
    }
}
