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

//! v1.4.17's Shield surface: ShieldClaim pays only its sender, in ETH or
//! NOX; settlementFee's bool argument is 0 or 1.

use nonos_settle_proto::{claim_ok, kind_of, view, view_calldata};

fn word(tail: &[u8]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[32 - tail.len()..].copy_from_slice(tail);
    w
}

fn claim(asset: u8, to: [u8; 20]) -> Vec<u8> {
    [&0x881a1ce0u32.to_be_bytes()[..], &word(&[asset]), &word(&to)].concat()
}

#[test]
fn a_claim_pays_only_its_sender_in_eth_or_nox() {
    let me = [7; 20];
    for asset in [0, 1] {
        assert!(claim_ok(&claim(asset, me), &me));
        assert_eq!(kind_of(20, &claim(asset, me)).map(|k| k.id), Some(20));
    }
    assert!(!claim_ok(&claim(2, me), &me), "another asset");
    assert!(!claim_ok(&claim(1, [8; 20]), &me), "paid to someone else");
    assert!(!claim_ok(&claim(1, me)[..67], &me), "short");
    assert!(!claim_ok(&[claim(1, me), vec![0; 32]].concat(), &me), "a trailing word");
    let mut dirty = claim(1, me);
    dirty[4 + 32 + 11] = 1;
    assert!(!claim_ok(&dirty, &me), "an address word with high bytes");
}

#[test]
fn settlement_fees_bool_is_strict() {
    let v = view(75).unwrap();
    let args = |b: u8| [word(&[1]), word(&[9]), word(&[3]), word(&[b])];
    assert!(view_calldata(v, &args(0)).is_some());
    assert!(view_calldata(v, &args(1)).is_some());
    assert_eq!(view_calldata(v, &args(2)), None);
    assert_eq!(view_calldata(view(61).unwrap(), &[]).unwrap(), vec![0x92, 0x2f, 0x2e, 0xe9]);
}
