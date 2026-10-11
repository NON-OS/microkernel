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

//! The escrow pin word init writes for an escrow install (plan/27).

use crate::install::escrow_pin::{escrow_pin, EscrowPin, NotEscrowPin, ESCROW_PIN_LEN};
use crate::install::market_arg::market_arg;

pub(super) const THREE_ROOT: &str =
    "7adc7dc20cfc847b03a2a898702639fe91196acb0449b051618444e71d7307b1";
pub(super) const ONE_ROOT: &str =
    "68cecbd0edf0b408e0c47497073daad93fd5ec517d410253545af93335c8f56c";
const ORDER: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

pub(super) fn sample(len: usize, mul: usize, add: usize) -> Vec<u8> {
    (0..len).map(|i| ((i * mul + add) % 256) as u8).collect()
}

pub(super) fn word(root: &str, len: u64) -> Vec<u8> {
    format!("e{root}{len:016x}{ORDER}").into_bytes()
}

pub(super) fn bytes32(hex: &str) -> [u8; 32] {
    core::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
}

#[test]
fn the_pin_word_reads_back_its_three_fields() {
    let w = word(THREE_ROOT, 2500);
    assert_eq!(w.len(), ESCROW_PIN_LEN);
    let pin = escrow_pin(&w).unwrap();
    let want =
        EscrowPin { chunk_root: bytes32(THREE_ROOT), content_len: 2500, order_id: bytes32(ORDER) };
    assert_eq!(pin, want);
}

#[test]
fn any_other_word_is_refused() {
    let good = word(ONE_ROOT, 1024);
    let mut upper = good.clone();
    upper[3] = upper[3].to_ascii_uppercase();
    let mut tag = good.clone();
    tag[0] = b'E';
    let short = &good[..ESCROW_PIN_LEN - 1];
    let long = [&good[..], b"0"].concat();
    let blake3 = good[1..65].to_vec();
    for bad in [&upper[..], &tag, short, &long, &blake3, b"-", b""] {
        assert_eq!(escrow_pin(bad), Err(NotEscrowPin));
    }
    assert_ne!(upper, good, "the sample digit must be a letter to test case");
    assert!(market_arg(&good).is_err(), "an escrow pin is not a market word either");
}
