// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A station over a chip that restarts, what the net stack and the panel
//! ask of it, and what the air saw it send.

use super::restart::Restarts;
use crate::soft::air_new::air;
use crate::soft::ap::SimAp;
use crate::soft::env::SimEnv;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use nonos_wifi_core::netif::wire::{HDR_LEN, MAGIC_NNET, OP_TX_PACKET, VERSION};
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;
use nonos_wifi_core::softmac::SoftStation;

pub type Restarted = SoftStation<Restarts, SimEnv>;

/// A station whose chip restarts at radio call `at` (never, for 0).
pub fn station(at: u64) -> Restarted {
    let mut ap = SimAp::new(SSID, PASS, 6, &RSNE_PSK);
    ap.chatter = true;
    let air = air(ap);
    let (clock, made) = (air.clock.clone(), air.addr.0);
    let chip = Restarts { air, at, calls: 0, made };
    SoftStation::new(Some(chip), SimEnv { clock, draws: 0 }, 0)
}

/// Join the AP; the status the panel would see.
pub fn connect(st: &mut Restarted) -> i32 {
    let mut req = WIFI_MAGIC.to_le_bytes().to_vec();
    req.extend_from_slice(&[1, 0, 7, 0, 0, 0]);
    req.extend_from_slice(
        &[&[SSID.len() as u8][..], SSID, &[PASS.len() as u8], PASS, &[0]].concat(),
    );
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    crate::soft::wire::code(&out[10..n])
}

/// One IPv4 packet from the net stack to the AP's network.
pub fn send_ip(st: &mut Restarted) {
    let mut req = vec![0u8; HDR_LEN];
    req[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&OP_TX_PACKET.to_le_bytes());
    let eth = [&[2, 0, 0, 0, 0, 9][..], &[2, 0, 0, 0, 0, 2], &[8, 0], b"secret payload"].concat();
    req[16..20].copy_from_slice(&(eth.len() as u32).to_le_bytes());
    req.extend_from_slice(&eth);
    st.handle(&req, &mut vec![0u8; MAX_RESPONSE]);
}

/// Data frames with a body: how many sealed, and those clear but not EAPOL.
pub fn seen(sent: &[Vec<u8>]) -> (usize, Vec<Vec<u8>>) {
    let data = sent.iter().filter(|f| (f[0] >> 2) & 3 == 2 && f[0] & 0x40 == 0);
    let (sealed, clear): (Vec<_>, Vec<_>) = data.partition(|f| f[1] & 0x40 != 0);
    let hdr = |f: &Vec<u8>| if f[0] & 0x80 != 0 { 26 } else { 24 };
    let eapol =
        |f: &Vec<u8>| f.get(hdr(f)..hdr(f) + 8) == Some(&[0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e]);
    (sealed.len(), clear.into_iter().filter(|f| !eapol(f)).cloned().collect())
}
