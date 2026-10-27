// NONOS Operating System (AGPL-3.0-or-later)

//! Talking to the firmware: command packets on the firmware queue, and the
//! register mailbox for the few commands rtw89 sends that way.

pub mod init_cmds;
pub mod packet;
pub mod reg;
pub mod send;

/// The firmware's per-download counters (`rtw89_fw_info`'s `h2c_seq`,
/// `h2c_counter`, `c2h_counter`), zeroed once a download completes.
#[derive(Default)]
pub struct Fw {
    pub h2c_seq: u8,
    pub mailbox: reg::Counters,
}
