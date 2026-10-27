// NONOS Operating System (AGPL-3.0-or-later)

//! What DACK found, `struct rtw89_dack_info` (Linux v6.12 rtw89 `core.h`):
//! each wait's outcome and the values read back after it.

/// `struct rtw89_dack_info`, what the calibration found.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DackInfo {
    pub done: bool,
    pub drck_timeout: bool,
    pub addck_timeout: [bool; 2],
    pub msbk_timeout: [bool; 2],
    pub dadck_timeout: [bool; 2],
    pub addck_d: [[u32; 2]; 2],
    pub msbk: [[[u8; 16]; 2]; 2],
    pub biask: [[u32; 2]; 2],
    pub dadck: [[u32; 2]; 2],
}

impl DackInfo {
    /// Whether any wait of the calibration ran out.
    pub fn timed_out(&self) -> bool {
        let any = |a: [bool; 2]| a[0] || a[1];
        self.drck_timeout
            || any(self.addck_timeout)
            || any(self.msbk_timeout)
            || any(self.dadck_timeout)
    }
}
