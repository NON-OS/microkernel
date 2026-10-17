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
//! W2: on a network offering WPA3 and WPA2, a forger who refuses every SAE
//! commit does not end the join, and the join never turns to WPA2. It runs
//! out its time with the forged refusal as the reason the panel shows.

use crate::assoc::{run_join, Outcome, Radio};
use nonos_wifi_core::mlme::{Entropy, JoinRequest, MlmeFailure, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::SaeFailure;

const OUR: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 0xAA];

/// A beacon offering WPA2-PSK and WPA3-SAE side by side, MFP capable.
fn transition_beacon() -> Vec<u8> {
    let mut f = vec![0x80, 0x00, 0, 0];
    f.extend_from_slice(&[0xFF; 6]);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&[0; 2 + 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04, 0, 3, b'n', b'e', b't', 3, 1, 6, 48, 24]);
    f.extend_from_slice(&[1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 4]);
    f.extend_from_slice(&[2, 0, 0x00, 0x0F, 0xAC, 2, 0x00, 0x0F, 0xAC, 8, 0x80, 0]);
    f
}

/// Answers each frame the station sends with an SAE refusal "from the AP".
struct Forger(Vec<Vec<u8>>, usize);
impl Radio for Forger {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.0.push(mpdu.to_vec());
        self.1 += 1;
        true
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<usize> {
        self.1 = self.1.checked_sub(1)?;
        let f = [&[0xB0, 0, 0, 0][..], &OUR, &AP, &AP, &[0, 0, 3, 0, 1, 0, 1, 0]].concat();
        out.get_mut(..f.len())?.copy_from_slice(&f);
        Some(f.len())
    }
}

#[test]
fn a_forged_sae_refusal_neither_ends_the_join_nor_turns_it_to_wpa2() {
    let mut radio = Forger(Vec::new(), 0);
    let entropy = Entropy { snonce: [0x11; 32], sae: [0x22; SAE_ENTROPY], rekey: [0x6B; 32] };
    let (ssid, passphrase, policy) = (&b"net"[..], &b"password123"[..], JoinPolicy::ANY);
    let req = JoinRequest { our_mac: OUR, ssid, passphrase, policy, entropy };
    let mut t = 0u64;
    let mut clock = || (t += 50, t).1;
    let report = run_join(&mut radio, &req, &transition_beacon(), 8_000, &mut clock);
    assert!(matches!(report.outcome, Outcome::TimedOut), "run to its deadline, not refused");
    assert_eq!(report.failure, Some(MlmeFailure::Sae(SaeFailure::Rejected(1))));
    assert_eq!(report.akm.map(|a| a.suite_type()), Some(8));
    assert!(radio.0.len() > 1, "the commit was resent on the clock");
    let alg = |f: &Vec<u8>| u16::from_le_bytes([f[24], f[25]]);
    assert!(radio.0.iter().all(|f| f[0] == 0xB0 && alg(f) == 3), "SAE only, never WPA2");
}
