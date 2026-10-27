// NONOS Operating System (AGPL-3.0-or-later)
//! The shared station over the driver over the modelled chip, with a WPA2
//! access point in the air, spoken to the way the panels speak to it.

use crate::model::{air, rig, Card};
use crate::radio::Station;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::station::CapsuleEnv;
use nonos_wifi_core::netif::wire::{HDR_LEN, MAGIC_NNET, VERSION};
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;
use nonos_wifi_core::softmac::SoftStation;

pub const SSID: &[u8] = b"ThisIsASSID";
pub const PASS: &[u8] = b"ThisIsAPassword";

pub type Sta = SoftStation<Station<Card, crate::model::bus::Mem>, CapsuleEnv>;

/// A station on a ready chip, an AP on `channel` in the air.
pub fn on_air(channel: u8) -> (Card, Sta) {
    let (card, chip) = rig::ready();
    air::attach(&mut card.0.borrow_mut(), SimAp::new(SSID, PASS, channel, &RSNE_PSK));
    (card, SoftStation::new(Some(Station::new(chip)), CapsuleEnv, 0))
}

/// The reply body of control `op`.
pub fn control(st: &mut Sta, op: u16, body: &[u8]) -> Vec<u8> {
    let mut req = WIFI_MAGIC.to_le_bytes().to_vec();
    req.extend_from_slice(&op.to_le_bytes());
    req.extend_from_slice(&77u32.to_le_bytes());
    req.extend_from_slice(body);
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    out[10..n].to_vec()
}

pub fn connect(st: &mut Sta, ssid: &[u8], pass: &[u8]) -> Vec<u8> {
    let mut b = vec![ssid.len() as u8];
    b.extend_from_slice(ssid);
    b.push(pass.len() as u8);
    b.extend_from_slice(pass);
    b.push(0);
    control(st, 1, &b)
}

/// net_core's request `op`; the reply's status and payload after it.
pub fn netif(st: &mut Sta, op: u16, payload: &[u8]) -> (i32, Vec<u8>) {
    let mut req = vec![0u8; HDR_LEN];
    req[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&op.to_le_bytes());
    req[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    req.extend_from_slice(payload);
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    let status = i32::from_le_bytes(out[HDR_LEN..HDR_LEN + 4].try_into().unwrap());
    (status, out[HDR_LEN + 4..n].to_vec())
}

pub fn code(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[..4].try_into().unwrap())
}
