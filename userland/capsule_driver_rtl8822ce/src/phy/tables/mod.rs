// NONOS Operating System (AGPL-3.0-or-later)

//! The RTL8822CE's baseband, AGC, calibration-init and RF path A and B
//! tables: Realtek's data from rtw88's rtw8822c_table.c (Linux v6.12), under
//! the BSD-3-Clause option of its "GPL-2.0 OR BSD-3-Clause" licence. Each
//! file is written by tools/nonos-rtw88-tables from that one file, held to
//! its sha256, carries Realtek's notice and the licence text, and is the
//! vendor's word stream unchanged; `--check` says whether they still match.
//! The PHY-MAC table (`rtw8822c_mac`) is empty in that file.

mod agc;
mod bb;
mod cal_init;
mod rf_a;
mod rf_b;

pub use agc::RTL8822C_AGC;
pub use bb::RTL8822C_BB;
pub use cal_init::RTL8822C_CAL_INIT;
pub use rf_a::RTL8822C_RF_A;
pub use rf_b::RTL8822C_RF_B;
