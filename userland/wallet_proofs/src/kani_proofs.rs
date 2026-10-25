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

//! Kani harnesses: the wallet read helpers meet their Lean specification for
//! every input, not just the sampled vectors. Each harness mirrors a theorem in
//! `verification/lean/Nonos/Wallet*.lean`, but runs against the shipping Rust.

use crate::nox::calldata_addr::calldata_addr;
use crate::nox::q32_to_u128::q32_to_u128;

/// `WalletNoxCalldata`: selector in [0,4), zero padding in [4,16), address
/// right-aligned in [16,36) for every selector and address.
#[kani::proof]
fn calldata_layout_is_exact() {
    let selector: [u8; 4] = kani::any();
    let addr: [u8; 20] = kani::any();
    let cd = calldata_addr(&selector, &addr);
    let mut i = 0;
    while i < 4 {
        assert!(cd[i] == selector[i]);
        i += 1;
    }
    while i < 16 {
        assert!(cd[i] == 0);
        i += 1;
    }
    while i < 36 {
        assert!(cd[i] == addr[i - 16]);
        i += 1;
    }
}

/// `WalletQuantity`: a word with any non-zero high byte is refused, and an
/// accepted word decodes to the big-endian value of its low sixteen bytes.
#[kani::proof]
fn quantity_guard_and_value() {
    let w: [u8; 32] = kani::any();
    let mut high_nonzero = false;
    let mut i = 0;
    while i < 16 {
        if w[i] != 0 {
            high_nonzero = true;
        }
        i += 1;
    }
    let got = q32_to_u128(&w);
    if high_nonzero {
        assert!(got.is_none());
    } else {
        let mut expected: u128 = 0;
        let mut j = 16;
        while j < 32 {
            expected = (expected << 8) | w[j] as u128;
            j += 1;
        }
        assert!(got == Some(expected));
    }
}
