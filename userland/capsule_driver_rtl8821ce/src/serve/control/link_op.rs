/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The link op: whether the radio is associated and with which network, as
//! `[associated u8][bssid 6][ssid_len u8][ssid]` after the header. The panels
//! read it to show the connection whoever made it (a panel, or net_core
//! joining a saved network at boot).

use nonos_wifi_core::netif::LinkPort;

use super::super::connect::Session;
use super::super::radio::Radio;
use super::WIFI_HDR;

pub(super) fn link_reply(
    radio: &Radio,
    session: Option<&Session>,
    out: &mut [u8],
) -> Option<usize> {
    let up = matches!(radio, Radio::Up(up) if up.link.link_up());
    let fixed = WIFI_HDR + 8;
    out[WIFI_HDR..fixed].fill(0);
    let Some(s) = session.filter(|_| up) else {
        return Some(fixed);
    };
    let len = (s.ssid_len as usize).min(s.ssid.len());
    out[WIFI_HDR] = 1;
    out[WIFI_HDR + 1..WIFI_HDR + 7].copy_from_slice(&s.bssid);
    out[fixed - 1] = len as u8;
    out[fixed..fixed + len].copy_from_slice(&s.ssid[..len]);
    Some(fixed + len)
}
