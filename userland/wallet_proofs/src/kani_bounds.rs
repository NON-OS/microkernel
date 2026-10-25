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

//! Kani harnesses: the APR never panics on any pool, and a hex digit is below sixteen.

use crate::nox::apr_bps::apr_bps;

/// `WalletNoxApr`: an empty pool never divides. Returns `None`, never a panic.
#[kani::proof]
fn apr_empty_pool_is_none() {
    let emission: u128 = kani::any();
    assert!(apr_bps(emission, 0).is_none());
}

/// `WalletNoxApr`: the checked arithmetic never overflows or panics for any
/// emission and stake; an out-of-range result is folded to `None`.
#[kani::proof]
fn apr_never_panics() {
    let emission: u128 = kani::any();
    let total: u128 = kani::any();
    let _ = apr_bps(emission, total);
}

/// `WalletHex`: an accepted nibble is always a real hex value below sixteen.
#[kani::proof]
fn hex_digit_in_range() {
    let code: u32 = kani::any();
    if let Some(v) = crate::hex_digit::hex_digit(code) {
        assert!(v < 16);
    }
}
