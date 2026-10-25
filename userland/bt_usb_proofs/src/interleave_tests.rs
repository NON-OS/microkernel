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
//! ACL from the controller with other links' fragments between ours (Vol
//! 3, Part A, 7.2; Vol 4, Part E, 5.4.2): only our handle's are joined, so
//! another link's never lands in, ends or breaks our PDU. A start with no
//! end, or fragments that overrun, costs that PDU alone.

use crate::air::{Act, HANDLE};
use crate::fixture::{air, keyboard, link};
use crate::interleave_sim::{between, cut, keys, Rng, SEEDS};
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::hci::{acl, Boundary};

#[test]
fn only_our_links_fragments_are_joined() {
    for seed in SEEDS {
        let mut rng = Rng(seed);
        let (mut host, ours, mut env) = keyboard();
        let (mut script, mut want) = (Vec::new(), Vec::new());
        for round in 0..9u8 {
            let key = 0x06 + round;
            let mut packets = match round % 3 {
                0 => Vec::new(),
                // A start whose PDU never ends: a later start replaces it.
                1 => {
                    let pdu = keys(0x30);
                    let n = 1 + rng.below(pdu.len() - 1);
                    vec![acl(HANDLE, Boundary::FirstFlushable, &pdu[..n]).unwrap()]
                }
                // Fragments past the stated length: the PDU is dropped.
                _ => {
                    let mut p = cut(&mut rng, HANDLE, &keys(0x31));
                    let last = p.last_mut().unwrap();
                    last.push(0);
                    let n = u16::from_le_bytes([last[2], last[3]]) + 1;
                    last[2..4].copy_from_slice(&n.to_le_bytes());
                    p
                }
            };
            let press = cut(&mut rng, HANDLE, &keys(key));
            packets.extend(between(&mut rng, press));
            let release = cut(&mut rng, HANDLE, &keys(0));
            packets.extend(between(&mut rng, release));
            script.push(Act::Acl(packets));
            want.extend([Input::KeyDown(key), Input::KeyUp(key)]);
        }
        script.push(Act::Drop(0x13));
        air(&mut host).script.extend(script);
        link(&mut host, &ours, &mut env).unwrap();
        assert_eq!(env.input, want, "seed {seed:#x}");
    }
}
