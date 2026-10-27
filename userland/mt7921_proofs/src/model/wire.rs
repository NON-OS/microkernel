// NONOS Operating System (AGPL-3.0-or-later)

//! Speaking to the driver's station as its callers do: the Wi-Fi control
//! family the panels send through `nonos_wifi_client`, and net.core's link
//! protocol, each through the driver's own `serve::answer` and its gate.

use nonos_wifi_core::netif::wire::{HDR_LEN, MAGIC_NNET, VERSION};
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;

use super::rig::Station;
use crate::serve::answer;

/// Who asks: the panel's pid, and net.core's (`set_net_core`).
pub const PANEL: u32 = 41;
pub const NET_CORE: u32 = 42;

/// The reply body of control `op` (after the 10-octet header).
pub fn control(st: &mut Station, op: u16, body: &[u8]) -> Vec<u8> {
    let mut req = WIFI_MAGIC.to_le_bytes().to_vec();
    req.extend_from_slice(&op.to_le_bytes());
    req.extend_from_slice(&77u32.to_le_bytes());
    req.extend_from_slice(body);
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = answer(st, &req, PANEL, &mut out);
    out[10..n].to_vec()
}

pub fn connect(st: &mut Station, ssid: &[u8], pass: &[u8]) -> Vec<u8> {
    let mut b = vec![ssid.len() as u8];
    b.extend_from_slice(ssid);
    b.push(pass.len() as u8);
    b.extend_from_slice(pass);
    b.push(0);
    control(st, 1, &b)
}

pub fn code(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[..4].try_into().unwrap())
}

/// The names in a scan reply.
pub fn scan_names(st: &mut Station) -> Vec<Vec<u8>> {
    let r = control(st, 3, &[]);
    let (mut names, mut at) = (Vec::new(), 13);
    for _ in 0..r[12] {
        let len = r[at + 2] as usize;
        names.push(r[at + 3..at + 3 + len].to_vec());
        at += 3 + len;
    }
    names
}

/// Link request `op` from `sender`; the reply's status and what follows.
pub fn netif(st: &mut Station, sender: u32, op: u16, payload: &[u8]) -> (i32, Vec<u8>) {
    let mut req = vec![0u8; HDR_LEN];
    req[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&op.to_le_bytes());
    req[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    req.extend_from_slice(payload);
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = answer(st, &req, sender, &mut out);
    let status = i32::from_le_bytes(out[HDR_LEN..HDR_LEN + 4].try_into().unwrap());
    (status, out[HDR_LEN + 4..n].to_vec())
}
