// NONOS Operating System (AGPL-3.0-or-later)

//! The baseband and radio: register access, rtw89's parameter tables, and
//! the PHY bring-up `rtw89_core_start` runs after the MAC init.

pub mod bb;
pub mod ch_bb;
pub mod ch_ctrl;
pub mod ch_gain;
pub mod ch_rf;
pub mod ch_rf18;
pub mod dm_hw;
pub mod gain;
pub mod headline;
pub mod init_bb;
pub mod init_rf;
pub mod lck;
pub mod nctl;
pub mod reset;
pub mod rf;
pub mod rf_read;
pub mod rpl;
pub mod rx_path;
pub mod state;
pub mod table;
pub mod trim;
pub mod txpwr;
pub mod txpwr_ref;
