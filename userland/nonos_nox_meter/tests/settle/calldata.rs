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

use nonos_nox_meter::settle::{close_data, settle_data, topup_data};
use nonos_nox_meter::settle::{CLOSE_SELECTOR, SETTLE_SELECTOR, TOPUP_SELECTOR};

use super::fixture::{channel, METER};
use sha3::{Digest, Keccak256};

const VOUCHER: &str = "(bytes32,address,address,bytes32,bytes32,uint256,uint64,uint64,bytes32)";

fn word(v: u64) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[24..].copy_from_slice(&v.to_be_bytes());
    w
}

#[test]
fn selectors_are_their_signatures() {
    let sel = |s: &str| <[u8; 4]>::try_from(&Keccak256::digest(s.as_bytes())[..4]).unwrap();
    assert_eq!(SETTLE_SELECTOR, sel(&format!("settle({VOUCHER}[],bytes[])")));
    assert_eq!(TOPUP_SELECTOR, sel("topUp(bytes32,uint256)"));
    assert_eq!(CLOSE_SELECTOR, sel("close(bytes32)"));
}

#[test]
fn settle_is_two_arrays_in_standard_abi() {
    let (a, b) = (channel(1, 300, [0x11; 65]), channel(2, 40, [0x22; 65]));
    let signed = [(a.voucher(&METER), a.sig), (b.voucher(&METER), b.sig)];
    let d = settle_data(&signed);
    let w = |i: usize| &d[4 + 32 * i..4 + 32 * (i + 1)];
    assert_eq!((w(0), w(1), w(2)), (&word(64)[..], &word(64 + 32 + 2 * 288)[..], &word(2)[..]));
    for (i, (v, _)) in signed.iter().enumerate() {
        for (j, field) in v.words().iter().enumerate() {
            assert_eq!(w(3 + 9 * i + j), &field[..], "voucher {i} word {j}");
        }
    }
    let sigs = 3 + 18;
    assert_eq!(
        (w(sigs), w(sigs + 1), w(sigs + 2)),
        (&word(2)[..], &word(64)[..], &word(64 + 128)[..])
    );
    for (i, (_, sig)) in signed.iter().enumerate() {
        let at = sigs + 3 + 4 * i;
        assert_eq!(w(at), &word(65)[..]);
        let bytes = &d[4 + 32 * (at + 1)..4 + 32 * (at + 4)];
        assert_eq!((&bytes[..65], &bytes[65..]), (&sig[..], &[0u8; 31][..]));
    }
    assert_eq!(d.len(), 4 + 32 * (sigs + 3 + 8));
}

#[test]
fn topup_and_close_name_the_channel() {
    let t = topup_data(&[7; 32], 500);
    assert_eq!((&t[4..36], &t[52..]), (&[7u8; 32][..], &500u128.to_be_bytes()[..]));
    assert_eq!(&close_data(&[7; 32])[4..], &[7u8; 32][..]);
}
