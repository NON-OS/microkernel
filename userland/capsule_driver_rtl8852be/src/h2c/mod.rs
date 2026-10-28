// NONOS Operating System (AGPL-3.0-or-later)

//! Talking to the firmware: command packets on the firmware queue, and the
//! register mailbox for the few commands rtw89 sends that way.

pub mod cam;
pub mod cam_keys;
pub mod cctl;
pub mod cctl_tbl;
pub mod init_cmds;
pub mod media;
pub mod packet;
pub mod post;
pub mod ra;
pub mod reg;
pub mod sec_cam;
pub mod send;

/// The firmware's per-download counters (`rtw89_fw_info`'s `h2c_seq`,
/// `h2c_counter`, `c2h_counter`), zeroed once a download completes.
#[derive(Default)]
pub struct Fw {
    pub h2c_seq: u8,
    pub mailbox: reg::Counters,
}
