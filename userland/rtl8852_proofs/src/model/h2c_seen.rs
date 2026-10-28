// NONOS Operating System (AGPL-3.0-or-later)
//! What the firmware learns from the commands it takes that the model's
//! receive filter and keys need: address CAM entry 0's station address,
//! BSSID and key slots (`rtw89_fw_h2c_cam`, laid out as
//! `rtw89_cam_fill_addr_cam_info` writes it), and the security CAM.

use super::cpu::H2c;
use super::state::State;

pub fn took(s: &mut State, c: &H2c) {
    if c.is(1, 0xa, 1) {
        if let Err(why) = s.sec.sec_upd(&c.body) {
            s.violations.push(why);
        }
    }
    if !c.is(1, 6, 0) {
        return;
    }
    s.sec.cam(&c.body);
    let Some(b) = c.body.get(16..28) else {
        return s.violations.push(format!("cam body of {}", c.body.len()));
    };
    let sma: [u8; 6] = b[..6].try_into().unwrap();
    let bssid: [u8; 6] = b[6..12].try_into().unwrap();
    s.cam = Some((sma, bssid));
}
